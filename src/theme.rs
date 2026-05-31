use std::collections::HashMap;
use std::path::PathBuf;

fn find_theme(pack: &str) -> PathBuf {
    let local = PathBuf::from("themes/colors");
    let fallback = dirs::home_dir()
        .unwrap()
        .join(".config/ivylink/themes/colors");

    for dir in [local, fallback] {
        if !dir.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_string_lossy().to_lowercase();
            if filename == format!("{}.json", pack.to_lowercase()) {
                return entry.path();
            }
        }
    }

    panic!("theme file not found for '{}'", pack);
}

pub fn apply(config_path: PathBuf, pack: &str, variant: &str) {
    let theme_path = find_theme(pack);
    let content = std::fs::read_to_string(&theme_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let colors = &json["variants"][variant]["color"];
    let mut vars: HashMap<String, String> = HashMap::new();

    if let Some(color_map) = colors.as_object() {
        for (key, value) in color_map {
            let hex = value.as_str().unwrap().to_string();
            vars.insert(key.clone(), hex.clone());
            // also insert raw version (strip the #)
            vars.insert(
                format!("{}_raw", key),
                hex.trim_start_matches('#').to_string(),
            );
        }
    }
    let content_kdl = std::fs::read_to_string(&config_path).unwrap();
    let doc: kdl::KdlDocument = content_kdl.parse().unwrap();
    let themes = doc.get("themes").expect("no themes section in config");
    let children = themes.children().unwrap().nodes();
    let home = dirs::home_dir().unwrap();

    for node in children {
        let name = node.entries()[0].value().as_string().unwrap();
        let template = node.get("template").unwrap().as_string().unwrap();
        let output = node.get("output").unwrap().as_string().unwrap();

        // find template - check local first, then ~/.config/ivylink
        let template_path = if PathBuf::from(format!("themes/templates/{}", template)).exists() {
            PathBuf::from(format!("themes/templates/{}", template))
        } else {
            home.join(format!(".config/ivylink/themes/templates/{}", template))
        };

        if !template_path.exists() {
            eprintln!("error: template not found for '{}'", name);
            continue;
        }

        let mut content = std::fs::read_to_string(&template_path).unwrap();
        for (key, val) in &vars {
            content = content.replace(&format!("{{{{{}}}}}", key), val);
        }

        let output = output.replacen("~/", &format!("{}/", home.display()), 1);
        std::fs::create_dir_all(std::path::Path::new(&output).parent().unwrap()).unwrap();
        std::fs::write(&output, &content).unwrap();
        println!("applied: {}", name);
    }
}

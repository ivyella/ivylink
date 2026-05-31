use std::path::PathBuf;

pub fn apply(config_path: PathBuf) {
    let content = std::fs::read_to_string(&config_path).expect("could not read config file");
    let doc: kdl::KdlDocument = content.parse().expect("could not parse config");
    let symlinks = doc.get("symlinks").expect("no symlinks section in config");
    let children = symlinks.children().unwrap().nodes();
    let home = dirs::home_dir().unwrap();

    for node in children {
        let source = node.get("source").unwrap().as_string().unwrap();
        let target = node.get("target").unwrap().as_string().unwrap();
        let source = source.replacen("~/", &format!("{}/", home.display()), 1);
        let target = target.replacen("~/", &format!("{}/", home.display()), 1);
        match std::os::unix::fs::symlink(&source, &target) {
            Ok(_) => println!("linked: {} -> {}", source, target),
            Err(e) => eprintln!("error linking {}: {}", source, e),
        }
    }
}

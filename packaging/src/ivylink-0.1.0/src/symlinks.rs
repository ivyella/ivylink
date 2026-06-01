use std::collections::HashSet;
use std::path::PathBuf;

pub fn apply(config_path: PathBuf, old: &HashSet<String>) -> HashSet<String> {
    let content = std::fs::read_to_string(&config_path).expect("could not read config file");
    let doc: kdl::KdlDocument = content.parse().expect("could not parse config");
    let symlinks = doc.get("symlinks").expect("no symlinks section in config");
    let children = symlinks.children().unwrap().nodes();
    let home = dirs::home_dir().unwrap();
    let mut new_targets: HashSet<String> = HashSet::new();

    for node in children {
        let source = node.get("source").unwrap().as_string().unwrap();
        let target = node.get("target").unwrap().as_string().unwrap();
        let source = source.replacen("~/", &format!("{}/", home.display()), 1);
        let target = target.replacen("~/", &format!("{}/", home.display()), 1);

        if !old.contains(&target) || !std::path::Path::new(&target).exists() {
            match std::os::unix::fs::symlink(&source, &target) {
                Ok(_) => println!("linked: {} -> {}", source, target),
                Err(e) => eprintln!("error linking {}: {}", source, e),
            }
        } else {
            println!("skipping (already linked): {}", target);
        }
        new_targets.insert(target.clone());
    }
    for stale in old.difference(&new_targets) {
        std::fs::remove_file(stale)
            .unwrap_or_else(|e| eprintln!("error removing {}: {}", stale, e));
        println!("removed: {}", stale);
    }
    new_targets
}

use std::collections::HashSet;
use std::path::PathBuf;

pub fn apply(config_path: PathBuf, old: &HashSet<String>) -> HashSet<String> {
    let content = std::fs::read_to_string(&config_path).expect("could not read config file");
    let doc: kdl::KdlDocument = content.parse().expect("could not parse config");
    let services = doc.get("services").expect("no services section in config");
    let children = services.children().unwrap().nodes();
    let mut new_services: HashSet<String> = HashSet::new();

    for node in children {
        let name = node.name().value();
        let exec = node.get("exec").unwrap().as_string().unwrap();
        let restart = node.get("restart").unwrap().as_string().unwrap(); // and this!
        let part_of = node.get("partOf").unwrap().as_string().unwrap(); // and this!
        let wanted_by = node.get("wantedBy").unwrap().as_string().unwrap(); // and this!
        let service_content = format!(
            "[Unit]
Description={name}
PartOf={part_of}

[Service]
ExecStart={exec}
Restart={restart}
KillMode=process

[Install]
WantedBy={wanted_by}
"
        );
        let home = dirs::home_dir().unwrap();
        let service_dir = home.join(".config/systemd/user");
        std::fs::create_dir_all(&service_dir).unwrap();
        let service_path = service_dir.join(format!("{}.service", name));
        std::fs::write(&service_path, &service_content).unwrap();
        println!("wrote service: {}", service_path.display());
        new_services.insert(service_path.to_string_lossy().to_string());
    }
    new_services
}

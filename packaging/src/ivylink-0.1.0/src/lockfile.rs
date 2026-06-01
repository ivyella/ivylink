use std::collections::HashSet;

pub fn read() -> HashSet<String> {
    let home = dirs::home_dir().unwrap();
    let lockfile = home.join(".config/ivylink/ivylink.lock");

    if !lockfile.exists() {
        return HashSet::new();
    }

    std::fs::read_to_string(&lockfile)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

pub fn write(paths: &HashSet<String>) {
    let home = dirs::home_dir().unwrap();
    let dir = home.join(".config/ivylink");
    let lockfile = dir.join("ivylink.lock");

    std::fs::create_dir_all(&dir).unwrap();
    let contents = paths.iter().cloned().collect::<Vec<_>>().join("\n");
    std::fs::write(&lockfile, contents).unwrap();
}

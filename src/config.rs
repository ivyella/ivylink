use std::path::{Path, PathBuf};

pub fn find_config() -> PathBuf {
    let local = Path::new("ivylink.kdl");
    if local.exists() {
        return local.to_path_buf();
    } else {
        let home = dirs::home_dir().expect("could not find home directory");
        let fallback = home.join(".config/ivylink/ivylink.kdl");
        if fallback.exists() {
            return fallback.to_path_buf();
        }
        panic!(
            "no config file found — place ivylink.kdl in the current directory or ~/.config/ivylink/"
        );
    }
}

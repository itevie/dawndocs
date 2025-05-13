use std::process::Command;

use crate::{debug, util::copy_dir_all, Locations};

pub fn download_dawn_ui(locations: Locations) {
    let dawn_ui_cache_path = locations.cache.clone().join("./dawn-ui");
    if dawn_ui_cache_path.exists() && dawn_ui_cache_path.is_dir() {
        copy_dir_all(dawn_ui_cache_path, locations.src.join("./dawn-ui"))
            .expect("Failed to copy dawn-ui to cache");
        debug("Copied dawn-ui from cache");
    } else {
        debug("Cloning dawn-ui");
        let status = Command::new("git")
            .arg("clone")
            .arg("https://github.com/itevie/dawn-ui")
            .current_dir(locations.src.clone())
            .status()
            .expect("Failed to clone dawn-ui");
        if !status.success() {
            panic!("Failed to run git clone! Status: {}", status);
        }

        copy_dir_all(locations.src.join("./dawn-ui"), dawn_ui_cache_path)
            .expect("Failed to copy dawn-ui to cache");
    }
}

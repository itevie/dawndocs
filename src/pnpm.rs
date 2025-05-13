use std::process::Command;

use crate::{debug, Locations};

pub fn install(locations: Locations) {
    debug("Running pnpm install");
    let status = Command::new("pnpm")
        .arg("install")
        .current_dir(locations.gen.clone())
        .status()
        .expect("Failed to run pnpm install");
    if !status.success() {
        panic!("Failed to run pnpm install! Status: {}", status);
    }
}

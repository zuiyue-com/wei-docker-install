#[macro_use]
extern crate wei_log;

#[cfg(target_os = "windows")]
mod windows;

pub fn install() {
    #[cfg(target_os = "windows")]
    windows::install();
}

pub fn uninstall() {
    #[cfg(target_os = "windows")]
    windows::uninstall();
}

pub fn check() -> serde_json::Value {
    #[cfg(target_os = "windows")] {
        if !std::path::Path::new(
            &format!("{}/docker/Ubuntu.tar.gz", std::env::current_dir().unwrap().display())
        ).exists() {
            info!("Ubuntu.tar.gz not found in {}", &format!("{}/docker/Ubuntu.tar.gz", std::env::current_dir().unwrap().display()));
            windows::write_json("file_check", false);
        }
    
        if !std::path::Path::new(
            &format!("{}/docker/wsl_update_x64.msi", std::env::current_dir().unwrap().display())
        ).exists() {
            info!("wsl_update_x64.msi not found in {}", &format!("{}/docker/wsl_update_x64.msi", std::env::current_dir().unwrap().display()));
            windows::write_json("file_check", false);
        }
        return serde_json::from_str(&windows::docker_dat()).unwrap();
    }
    
}

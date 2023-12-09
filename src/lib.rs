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
    #[cfg(target_os = "windows")]
    return serde_json::from_str(&windows::docker_dat()).unwrap();
}

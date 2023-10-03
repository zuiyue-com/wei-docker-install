#[macro_use]
extern crate wei_log;

#[cfg(target_os = "windows")]
mod windows;

pub fn install() {
    #[cfg(target_os = "windows")]
    windows::install();
}

pub fn check() -> f32 {
    #[cfg(target_os = "windows")]
    return windows::check();
}

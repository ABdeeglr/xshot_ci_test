pub mod xshot_api;
pub mod xshot_linux;
pub mod xshot_win;

pub use xshot_api::ScreenshotHandler;

#[cfg(target_os = "linux")]
pub type PlatformShoter = xshot_linux::LinuxShoter;

#[cfg(target_os = "windows")]
pub type PlatformShoter = xshot_win::WinShoter;

pub fn get_handler() -> PlatformShoter {
    #[cfg(target_os = "linux")]
    {
        xshot_linux::LinuxShoter
    }

    #[cfg(target_os = "windows")]
    {
        xshot_win::WinShoter
    }
}

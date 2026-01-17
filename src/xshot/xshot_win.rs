use crate::xshot::xshot_api::ScreenshotHandler;

pub struct WinShoter;

impl ScreenshotHandler for WinShoter {
    fn capture_all(&self) {
        println!("Windows: 调用 GDI+ 截取全屏...");
    }
    fn capture_focus(&self) {
        println!("Windows: 调用 Desktop Duplication API 截取焦点...");
    }
}

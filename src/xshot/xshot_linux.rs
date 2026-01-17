use crate::xshot::xshot_api::ScreenshotHandler;

pub struct LinuxShoter;

impl ScreenshotHandler for LinuxShoter {
    fn capture_all(&self) {
        println!("Linux: 调用 Wayland 接口截取全屏...");
    }
    fn capture_focus(&self) {
        println!("Linux: 调用 X11/Portal 接口截取焦点...");
    }
}

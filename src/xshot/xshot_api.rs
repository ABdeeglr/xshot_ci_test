pub trait ScreenshotHandler {
    /// 截取全屏
    fn capture_all(&self);
    /// 截取当前活动窗口
    fn capture_focus(&self);
}

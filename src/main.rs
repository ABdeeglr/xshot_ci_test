mod xshot;
use xshot::ScreenshotHandler;

fn main() {

    let shoter = xshot::get_handler();
    
    shoter.capture_all();
    shoter.capture_focus();
}

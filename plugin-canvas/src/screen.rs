// Panics if not called from the main thread
#[cfg(target_os = "macos")]
pub fn screen_scale() -> f64 {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSScreen;

    let Some(main_thread_marker) = MainThreadMarker::new() else {
        panic!("screen_scale() must be called from the main thread");
    };

    if let Some(screen) = NSScreen::mainScreen(main_thread_marker) {
        screen.backingScaleFactor()
    } else {
        1.0
    }
}

#[cfg(target_os = "windows")]
pub fn screen_scale() -> f64 {
    // Could use `GetDpiForWindow` here, but that's only available from Windows 10
    let dpi = unsafe {
        use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, LOGPIXELSX, LOGPIXELSY, ReleaseDC};

        let hdc = GetDC(None);
        if hdc.is_invalid() {
            return 1.0;
        }

        let dpi = GetDeviceCaps(Some(hdc), LOGPIXELSX).min(GetDeviceCaps(Some(hdc), LOGPIXELSY));
        ReleaseDC(None, hdc);

        dpi
    };

    if dpi > 0 {
        dpi as f64 / 96.0
    } else {
        1.0
    }
}

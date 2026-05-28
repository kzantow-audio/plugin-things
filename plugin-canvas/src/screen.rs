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

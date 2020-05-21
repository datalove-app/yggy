#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;

#[cfg(target_os = "linux")]
mod linux;

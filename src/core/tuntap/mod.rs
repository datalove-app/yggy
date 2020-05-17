use crate::core::types::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;

#[cfg(target_os = "linux")]
mod linux;

const TUN_IPV6_HEADER_LENGTH: u8 = 40;

#[derive(Debug)]
pub struct TunAdapter {
    addr: Address,
    subnet: Subnet,
    mtu: MTU,
}

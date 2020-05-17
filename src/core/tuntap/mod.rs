use crate::core::types::*;

const TUN_IPV6_HEADER_LENGTH: u8 = 40;

#[derive(Debug)]
pub struct TunAdapter {
    addr: Address,
    subnet: Subnet,
    mtu: MTU,
}

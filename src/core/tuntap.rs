use crate::core::{error::Error, types::*};

const TUN_IPV6_HEADER_LENGTH: u8 = 40;

/// Represents a running TUN interface.
///
///
pub trait TunAdapter {
    fn name(&self) -> &str;

    fn mtu(&self) -> &MTU;

    fn start(&mut self);
}

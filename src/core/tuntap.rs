use crate::core::{error::Error, types::*};

const TUN_IPV6_HEADER_LENGTH: u8 = 40;

// TODO? look at https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

/// Represents a running TUN interface.
///
pub trait TunAdapter {
    fn name(&self) -> &str;

    fn mtu(&self) -> &MTU;

    fn start(&mut self);
}

///
/// TODO? is an actor that represents a connection (session?) with a remote peer
///     handles and forwards (streams?) all recv'd packets to to a listener
///     handles
pub trait TunConn {}

use super::{BoxPublicKey, Coords, Handle, MTU};
use std::time::Instant;

///
#[derive(Debug)]
pub struct SessionPingPong {
    /// The sender's permanent key.
    sender_perm_key: BoxPublicKey,
    /// Session key to use.
    sender_session_key: BoxPublicKey,
    /// Random number used to identify the session.
    handle: Handle,
    coords: Coords,
    timestamp: Instant,
    is_pong: bool,
    mtu: MTU,
}

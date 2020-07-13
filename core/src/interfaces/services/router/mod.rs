//! Handles routing packets to and from ourself.
//!
//! Send:
//! - Receive a packet from the adapter
//! - Look up session (if none exists, trigger a search)
//! - Hand off to session (which encrypts, etc)
//! - Session will pass it back to router.out, which hands it off to the self peer
//! - The self peer triggers a lookup to find which peer to send to next
//! - And then passes it to that's peer's peer.out function
//! - The peer.out function sends it over the wire to the matching peer

//! Recv:
//! - A packet comes in off the wire, and goes to a peer.handlePacket
//! - The peer does a lookup, sees no better peer than the self
//! - Hands it to the self peer.out, which passes it to router.in
//! - If it's dht/seach/etc. traffic, the router passes it to that part
//! - If it's an encapsulated IPv6 packet, the router looks up the session for it
//! - The packet is passed to the session, which decrypts it, router.recvPacket
//! - The router then runs some sanity checks before passing it to the adapter

pub mod search;
pub mod session;

pub use search::{Search, SearchManager};
pub use session::{Session, SessionManager};

use crate::{dev::*, interfaces::Core, types::wire};
use std::fmt::Debug;

/// Handles packets to/from self.
pub trait Router<C: Core>: Debug
where
    Self: Actor,
    Self: StreamHandler<wire::Traffic>,
    Self: StreamHandler<wire::ProtocolTraffic>,
{
    // type Interface: PeerInterface;
    type SearchManager: SearchManager<C>;
    type SessionManager: SessionManager<C>;

    fn reconfigure(&mut self);
}

pub mod messages {
    use super::*;
}

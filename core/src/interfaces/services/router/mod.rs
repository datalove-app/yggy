pub mod search;
pub mod session;

use crate::{interfaces::Core, types::wire};
use std::fmt::Debug;
use xactor::{Actor, StreamHandler};

/// Handles packets to/from self.
pub trait Router<C: Core>: Debug
where
    Self: Actor,
    Self: StreamHandler<wire::Traffic>,
    Self: StreamHandler<wire::ProtocolTraffic>,
{
    // type Interface: PeerInterface;
    type SearchManager: search::SearchManager<C>;
    type SessionManager: session::SessionManager<C>;

    fn reconfigure(&mut self);
}

pub mod messages {
    use super::*;
}

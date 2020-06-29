//!

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

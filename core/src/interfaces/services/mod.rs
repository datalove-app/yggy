//! Core protocol services.

pub mod peer;
pub mod router;
pub mod switch;

#[doc(inline)]
pub use peer::{Peer, PeerManager};
#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use switch::Switch;

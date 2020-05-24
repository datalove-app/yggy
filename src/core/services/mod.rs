//! Core protocol services.

pub mod link;
pub mod peer;
pub mod router;
pub mod switch;

#[doc(inline)]
pub use link::Link;
#[doc(inline)]
pub use peer::PeerManager;
#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use switch::Switch;

use xactor::*;

///
/// todo Link
pub trait Core
where
    Self: Actor,
{
}

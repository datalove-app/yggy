//! Core protocol service implementations.

mod peer;
mod router;
mod switch;

#[doc(inline)]
pub use peer::PeerManager;
#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use switch::Switch;

use crate::core_interfaces::Core as ICore;

///
#[derive(Debug)]
pub struct Core;

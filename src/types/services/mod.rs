//! Core protocol services.

mod router;
mod switch;

#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use switch::Switch;

///
pub trait Core<S: Switch, R: Router> /* P: Peers, L: Link */ {}

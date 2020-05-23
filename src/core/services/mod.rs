//! Core protocol services.

pub mod router;
pub mod session;
pub mod switch;

#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use session::{Session, SessionManager};
#[doc(inline)]
pub use switch::Switch;

///
pub trait Core<S: Switch, R: Router> /* P: Peers, L: Link */ {}

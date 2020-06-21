//! Core `yggy` service implementations.

mod peer;
mod router;
// mod search;
mod session;
// mod switch;

#[doc(inline)]
pub use peer::PeerManager;
#[doc(inline)]
pub use router::Router;
// #[doc(inline)]
// pub use search::SearchManager;
#[doc(inline)]
pub use session::SessionManager;
// #[doc(inline)]
// pub use switch::Switch;

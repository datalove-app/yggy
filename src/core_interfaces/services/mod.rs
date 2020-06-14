//! Core protocol services.

pub mod router;
pub mod search;
pub mod session;
pub mod switch;

#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use search::SearchManager;
#[doc(inline)]
pub use session::{Session, SessionManager};
#[doc(inline)]
pub use switch::Switch;

//!

mod search;
mod session;

#[doc(inline)]
pub use search::SearchManager;
#[doc(inline)]
pub use session::{Session, SessionManager};

use crate::core::{types::wire, Core};
use xactor::{Actor, StreamHandler};

///
///
pub trait Router<C: Core>
where
    Self: Actor,
    Self: StreamHandler<wire::Traffic>,
    Self: StreamHandler<wire::ProtocolTraffic>,
{
    // type IncomingTraffic:

    fn reconfigure(&mut self);
}

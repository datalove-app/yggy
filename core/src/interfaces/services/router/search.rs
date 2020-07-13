//! Manages searches for ...
//!
//! ? `Coords` and `SigningPublicKey`s for nodes for which the IPv6 is known

use crate::{dev::*, types::NodeID};

///
/// ?? Handle<StartSearch>
pub trait SearchManager<C: Core>: Sized {
    // ///
    // type Router: <C as Core>::Router;

    /// Information about an ongoing search.
    ///
    type Search: Search<C, Self>;

    fn reconfigure(&mut self);

    // fn new_search(&self, dest: NodeID, mask: NodeID) -> Result<&SearchInfo, Error>;
}

///
/// ?? can be polled until completion, producing a Session
pub trait Search<C: Core, S: SearchManager<C>>
// where
// Self: ActorFuture<Actor = Self, Output = Addr<S>> + Actor,
// S: Session,
{
}

// use super::Session;
use crate::{core_interfaces::Core, core_types::NodeID, error::Error};
use xactor::Actor;

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

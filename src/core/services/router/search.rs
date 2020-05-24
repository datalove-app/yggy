use super::Session;
use crate::{
    core::types::{NodeID, SearchInfo},
    error::Error,
};
use xactor::LocalService;

///
/// ?? Handle<StartSearch>
pub trait SearchManager
where
    Self: LocalService,
{
    /// Information about an ongoing search.
    ///
    // type Search: Search<S>;

    fn reconfigure(&mut self);

    fn start_search(&self, dest: NodeID, mask: NodeID) -> Result<&SearchInfo, Error>;
}

///
/// ?? can be polled until completion, producing a Session
pub trait Search
// where
// Self: ActorFuture<Actor = Self, Output = Addr<S>> + Actor,
// S: Session,
{
}

use super::Session;
use actix::prelude::*;

///
/// ?? Handle<StartSearch>
pub trait SearchManager<S>
where
    //     Self: SystemService,
    S: Session,
{
    /// Information about an ongoing search.
    ///
    type Search: Search<S>;
}

///
/// ?? can be polled until completion, producing a Session
pub trait Search<S>
where
    // Self: ActorFuture<Actor = Self, Output = Addr<S>> + Actor,
    S: Session,
{
}

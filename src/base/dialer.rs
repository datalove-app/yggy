use super::{
    types::{Address, NetworkID},
    Conn, Error,
};
use actix::prelude::*;
use async_trait::async_trait;

///
/// ?? Handle<...>
#[async_trait]
pub trait Dialer<C: Conn>
where
    Self: Actor + ActorFuture<Actor = Self, Output = C>,
{
    //
    //  TODO follow flow
    //      search `router` (`searches` map) for nodeid
    //          if exists, error
    //          if timeouts, error
    //          else,
    //              router.searches.newIterSearch, startSearch
    //              finish initing conn.session
    // async fn dial(&self, network_id: NetworkID, address: Address) -> Result<Self::Conn, Error>;
}

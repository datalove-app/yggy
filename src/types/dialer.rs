use super::{
    primitives::{Address, NetworkID},
    Conn,
};
use crate::error::Error;
use actix::prelude::*;
use async_trait::async_trait;

///
///
/// ? Handle<...>
pub trait Dialer<C: Conn> // where
//     Self: ActorFuture<Actor = Self, Output = C> + Actor,
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

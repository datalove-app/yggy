use super::{
    types::{Address, NetworkID},
    Conn, Core,
};
use crate::error::Error;
use async_trait::async_trait;
use xactor::Actor;

///
///
/// ? Handle<...>
pub trait Dialer<C: Core>
where
    Self: Actor,
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

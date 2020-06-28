use super::{Conn, Core};
use crate::{
    error::Error,
    types::{Address, NetworkID},
};
use async_trait::async_trait;
use std::convert::TryInto;

/// Represents a connection dialer.
///
/// ? Handle<...>
#[async_trait::async_trait]
pub trait Dialer<C: Core>: Send {
    //
    //  TODO follow flow
    //      search `router` (`searches` map) for nodeid
    //          if exists, error
    //          if timeouts, error
    //          else,
    //              router.searches.newIterSearch, startSearch
    //              finish initing conn.session
    async fn dial<A: TryInto<Address>>(
        &mut self,
        address: A,
        network: NetworkID,
    ) -> Result<C::Conn, Error>;
}

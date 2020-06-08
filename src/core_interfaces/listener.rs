use super::{Conn, Core};
use crate::error::Error;
use futures::Stream;
use xactor::Addr;

/// Represents a connection listener.
///
/// Produces a stream of `Conn`s.
///
/// Provided by core
///
/// ? Handle<...>
#[async_trait::async_trait]
pub trait Listener<C: Core>: Send + Sized
where
    Self: Stream<Item = Result<C::Conn, Error>>,
{
    async fn accept(&mut self) -> Result<C::Conn, Error>;

    // async fn bind() -> Result<Self, Error> {
    //     unimplemented!()
    // }
}

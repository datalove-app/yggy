use actix::prelude::*;
use tokio::io::{AsyncBufRead, AsyncWrite};

/// Represents an active connection session between the local node and a remote
/// node.
pub trait Conn
where
    Self: Actor + AsyncBufRead + AsyncWrite,
{
    const READ_BUFFER_SIZE: u16 = 1024;

    fn id(&self) -> &str;

    fn set_mtu(&mut self);
}

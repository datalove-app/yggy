// use crate::core_interfaces::Session;
use futures::io::{AsyncRead, AsyncWrite};

/// Represents an active connection session between the local node and a remote
/// node.
///
/// It posesses a read buffer that
///
/// ?? is a Port
/// todo <S: Session>
pub trait Conn: Send
where
    Self: AsyncRead + AsyncWrite,
{
    const READ_BUFFER_SIZE: u16 = 1024;

    fn id(&self) -> &str;

    fn set_mtu(&mut self);
}

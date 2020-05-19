use tokio::io::{AsyncBufRead, AsyncRead};

/// Represents an active connection session between the local node and a remote
/// node.
pub trait Conn: AsyncBufRead {
    const READ_BUFFER_SIZE: u16 = 1024;

    fn id(&self) -> &str;

    fn set_mtu(&mut self);
}

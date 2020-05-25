use crate::core::{types::ROOT_TIMEOUT, Core};
use std::time::Duration;
use xactor::{Actor, Handler, Message};

lazy_static! {
    /// Time to wait before closing the link.
    pub static ref CLOSE_TIMEOUT: Duration = ROOT_TIMEOUT * 2;
}

/// Time to wait before deciding a send is blocked.
pub const SEND_TIMEOUT: Duration = Duration::from_secs(1);
/// Time to wait before sending a keep-alive message if we have no real traffic
/// to send.
pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(2);
///
pub const STALL_TIMEOUT: Duration = Duration::from_secs(6);

///
/// TODO
/// Seems to handle traffic from addresses listed in `ListenAddresses`, restricted `AllowedEncryptionPublicKeys`.
pub trait Link<C: Core>
where
    Self: Actor,
{
    type Interface: LinkInterface<C, Self>;

    fn reconfigure(&mut self);
}

///
pub trait LinkInterface<C: Core, L: Link<C>>
where
    Self: Actor,
    Self: Handler<Notification>,
{
    ///
    type Reader: LinkReader;
    ///
    type Writer: LinkWriter;
}

///
pub trait LinkReader
where
    Self: Actor,
{
}

///
pub trait LinkWriter
where
    Self: Actor,
{
}

///
#[xactor::message(result = "()")]
#[derive(Clone, Copy, Debug)]
pub enum Notification {
    Sending,
    BlockedSend,
    Sent { size: usize, is_link_traffic: bool },
    Stalled,
    Reading,
    Read(usize),
    KeepAlive,
}

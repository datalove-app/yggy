use crate::{
    core_interfaces::{Core, Peer, PeerManager},
    core_types::{PeerURI, ROOT_TIMEOUT},
};
use futures::prelude::*;
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

/// Represents direct connections to peers, over some `LinkInterface` (TCP, UDP, AWDL, etc).
/// TODO tor?
/// Seems to handle traffic from addresses in the `Listen` configuration option,
/// restricted by the `AllowedEncryptionPublicKeys` option.
pub trait LinkManager<C: Core, P: PeerManager<C>>
where
    Self: Actor,
    Self: Handler<messages::Listen>,
{
    type Link: Link<C, P::Peer>;

    fn reconfigure(&mut self);
}

///
pub trait Link<C: Core, P: Peer<C>>
where
    Self: Actor,
    Self: Handler<messages::Notification>,
{
    // ///
    // type Reader: LinkReader;
    // ///
    // type Writer: LinkWriter;

    // ///
    // async fn split()
}

#[async_trait::async_trait]
pub trait LinkInterface {
    ///
    type Reader: AsyncRead;
    ///
    type Writer: AsyncWrite;

    // async fn listen()
    // fn split()
}

// ///
// pub trait LinkReader
// where
//     Self: Actor,
// {
// }

// ///
// pub trait LinkWriter
// where
//     Self: Actor,
// {
// }

pub mod messages {
    use super::*;

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

    #[derive(Debug)]
    pub struct Listen {
        addr: PeerURI,
    }

    #[async_trait::async_trait]
    impl xactor::Message for Listen {
        type Result = ();
    }
}

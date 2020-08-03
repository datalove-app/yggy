//!

use crate::{dev::*, interfaces::peer, types::PeerURI};

/// Represents direct connections to peers, over some `LinkInterface` (TCP, UDP, AWDL, etc).
/// TODO tor?
/// Seems to handle traffic from addresses in the `Listen` configuration option,
/// restricted by the `AllowedEncryptionPublicKeys` option.
pub trait LinkAdapter<C: Core>
where
    Self: Actor,
{
    fn reconfigure(&mut self);
}

///
pub trait Link<C: Core, L: LinkAdapter<C>>
where
    Self: Actor,
    // Self: PeerInterface,
    Self: Handler<messages::Notification>,
{
    // ///
    // async fn split()
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

///
/// TODO docs, is this necessary?
#[async_trait::async_trait]
pub trait LinkInterface: Actor {
    // ///
    // type Reader: AsyncRead; // ? Stream?
    // ///
    // type Writer: AsyncWrite; // ? Actor? Sink?

    fn out(intf: Addr<Self>);

    fn link_out(intf: Addr<Self>);

    fn close(intf: Addr<Self>);

    fn name(&self) -> &str;

    fn local(&self) -> &PeerURI;

    fn remote(&self) -> &PeerURI;

    fn interface_type(&self) -> &str;
}

pub mod messages {
    use super::*;

    ///
    #[xactor::message(result = "()")]
    #[derive(Clone, Copy, Debug)]
    pub enum Notification {
        Sending { size: usize, is_link_traffic: bool },
        BlockedSend,
        Sent { size: usize, is_link_traffic: bool },
        Stalled,
        Reading,
        Read(usize),
        KeepAlive,
    }

    // #[derive(Debug)]
    // #[xactor::message(result = "()")]
    // pub struct Listen {
    //     addr: PeerURI,
    // }

    // #[async_trait::async_trait]
    // impl xactor::Message for Listen {
    //     type Result = ();
    // }
}

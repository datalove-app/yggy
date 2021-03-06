//!

use crate::{dev::*, interfaces::peer, types::PeerURI};
use std::fmt::Debug;

/// Represents direct connections to peers, over some `LinkInterface` (TCP, UDP, AWDL, etc).
/// TODO tor?
/// Seems to handle traffic from addresses in the `Listen` configuration option,
/// restricted by the `AllowedEncryptionPublicKeys` option.
pub trait LinkManager<C: Core>: Sized
where
    Self: Debug,
{
    ///
    type Link: Link<C, Self>;

    fn reconfigure(&mut self);
}

///
pub trait Link<C: Core, L: LinkManager<C>>
where
    Self: Actor,
    Self: LinkInterface,
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
pub trait LinkInterface: Sized
where
    Self: Actor,
{
    ///
    type Inner: LinkInterfaceInner;
    // ///
    // type Reader: AsyncRead; // ? Stream?
    // ///
    // type Writer: AsyncWrite; // ? Actor? Sink?

    fn out<T: Wire>(intf: &mut Self::Inner, msg: T);

    fn link_out<T: Wire>(intf: &mut Self::Inner, msg: T);

    fn close(intf: &mut Self::Inner);

    fn name(&self) -> &str;

    fn local(&self) -> &PeerURI;

    fn remote(&self) -> &PeerURI;

    fn interface_type(&self) -> &str;
}

pub trait LinkInterfaceInner: Debug + Send {}
impl<L: LinkInterface<Inner = Addr<L>>> LinkInterfaceInner for Addr<L> {}

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

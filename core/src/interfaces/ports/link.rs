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

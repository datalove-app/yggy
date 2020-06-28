use crate::{
    interfaces::Core,
    types::{BoxPublicKey, BoxSharedKey, SigningPublicKey},
};
use futures::{io, prelude::*};
use std::{pin::Pin, task};
use xactor::{Actor, Addr, Handler, Message};

/// Represents peer
pub trait PeerManager<C: Core>
where
    Self: Actor,
    Self: Handler<messages::NewPeer>,
    Self: Handler<messages::Close>,
{
    ///
    type Peer: Peer<C, Self>;
}

/// Represents a
pub trait Peer<C: Core, P: PeerManager<C>>
where
    Self: Actor,
{
}

///
/// TODO
#[async_trait::async_trait]
pub trait PeerInterface {
    ///
    type Reader: AsyncRead; // ? Stream?
    ///
    type Writer: AsyncWrite; // ? Actor? Sink?

    // fn split()
}

pub mod messages {
    use super::*;

    /// Signals the creation of a new `Peer`.
    #[derive(Debug)]
    pub struct NewPeer {
        signing_pub_key: SigningPublicKey,
        box_pub_key: BoxPublicKey,
        box_shared_key: BoxSharedKey,
    }

    #[async_trait::async_trait]
    impl xactor::Message for NewPeer {
        type Result = ();
    }

    /// Signals the closing of a `Peer` connection.
    #[xactor::message(result = "()")]
    #[derive(Debug)]
    pub struct Close;
}

// ///
// #[derive(Debug, Default)]
// pub struct Peer {
//     pub_sign_key: SigningPublicKey,
//     pub_box_key: BoxPublicKey,
//     endpoint: PeerURI, // TODO protocol + endpoint + port
//     // socket:
//     uptime: Duration,
// }

// ///
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct PeerInfo {
//     key: SigningPublicKey,
//     locator: SwitchLocator,
//     port: SwitchPort,
//     degree: u64,
//     last_seen: Instant,
//     // msg: SwitchMessage,
//     is_blocked: bool,
// }

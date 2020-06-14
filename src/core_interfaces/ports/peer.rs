use crate::{
    core_interfaces::Core,
    core_types::{BoxPublicKey, BoxSharedKey, SigningPublicKey},
};
use xactor::{Actor, Addr, Handler, Message};

/// Represents peer
pub trait PeerManager<C: Core>
where
    Self: Actor,
    Self: Handler<messages::NewPeer>,
    Self: Handler<messages::Close>,
{
    type Peer: Peer<C>;
}

/// Represents a
pub trait Peer<C: Core>
where
    Self: Actor,
{
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

//! Types and traits related to connections to other nodes on the network.

use crate::{
    dev::*,
    interfaces::{link, router},
    types::{BoxPublicKey, BoxSecretKey, BoxSharedKey, SigningPublicKey},
};
use std::marker::PhantomData;

/// Represents ...
///
/// TODO docs
#[async_trait::async_trait]
pub trait PeerManager<C: Core>
where
    Self: Actor,
    Self: Handler<messages::NewPeer<C, Self>>,
    Self: Handler<messages::ClosePeer>,
{
    ///
    type Peer: Peer<C, Self>;

    ///
    type PeerInterface: Send;

    #[inline]
    async fn new_peer<L: IntoPeerInterface<C, Self>>(
        addr: &mut Addr<Self>,
        sig_pub: SigningPublicKey,
        box_pub: BoxPublicKey,
        link_shared: BoxSharedKey,
        intf_addr: Addr<L>,
    ) -> Result<Addr<Self::Peer>, Error>;
    // where
    //     Self::PeerInterface: From<Addr<L>>;

    #[inline]
    fn close_peer(addr: &mut Addr<Self>) -> Result<(), Error> {
        Ok(addr.send(messages::ClosePeer)?)
    }
}

/// Represents ...
#[async_trait::async_trait]
pub trait Peer<C: Core, P: PeerManager<C>>: Sized
where
    Self: Actor,
    Self: Handler<messages::HandlePacket>,
{
    // TODO &mut Addr?
    #[inline]
    async fn handle_packet<T>(mut addr: Addr<Self>, msg: T) -> Result<usize, Error>
    where
        T: Send + Into<messages::HandlePacket>,
    {
        Ok(addr.call(msg.into()).await??)
    }
}

pub trait IntoPeerInterface<C: Core, P: PeerManager<C>>: link::LinkInterface {
    fn into(addr: Addr<Self>) -> P::PeerInterface;
}

pub mod messages {
    use super::*;
    use derive_more::{From, Into};

    /// Signals the creation of a new `Peer` with the provided cryptographic keys.
    #[derive(Debug)]
    pub struct NewPeer<C: Core, P: PeerManager<C>> {
        sig_pub: SigningPublicKey,
        box_pub: BoxPublicKey,
        box_shared: BoxSharedKey,
        intf: P::PeerInterface,
        peer: PhantomData<P::Peer>,
    }

    impl<C: Core, P: PeerManager<C>> NewPeer<C, P> {
        #[inline]
        pub fn new(
            sig_pub: SigningPublicKey,
            box_pub: BoxPublicKey,
            box_shared: BoxSharedKey,
            intf: P::PeerInterface,
        ) -> Self {
            Self {
                sig_pub,
                box_pub,
                box_shared,
                intf,
                peer: PhantomData,
            }
        }
    }

    impl<C: Core, P: PeerManager<C>> Message for NewPeer<C, P> {
        type Result = Result<Addr<P::Peer>, Error>;
    }

    /// Signals the closing of a `Peer` connection.
    #[xactor::message(result = "()")]
    #[derive(Debug)]
    pub struct ClosePeer;

    #[xactor::message(result = "Result<usize, Error>")]
    #[derive(Debug, From, Into)]
    pub struct HandlePacket(wire::Packet);
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

//! Types and traits related to connections to other nodes on the network.

use crate::{
    dev::*,
    interfaces::{link, router},
    types::{BoxPublicKey, BoxSecretKey, BoxSharedKey, SigningPublicKey},
};
use std::{fmt::Debug, marker::PhantomData};

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

    #[inline]
    async fn new_peer(
        addr: &mut Addr<Self>,
        sig_pub: SigningPublicKey,
        box_pub: BoxPublicKey,
        link_shared: BoxSharedKey,
        intf: Box<dyn link::LinkInterfaceInner>,
    ) -> Result<Addr<Self::Peer>, Error> {
        let msg = messages::NewPeer::<C, Self>::new(sig_pub, box_pub, link_shared, intf);
        Ok(addr.call(msg).await??)
    }

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

pub mod messages {
    use super::*;
    use derive_more::{From, Into};

    /// Signals the creation of a new `Peer` with the provided cryptographic keys.
    #[derive(Debug)]
    pub struct NewPeer<C: Core, P: PeerManager<C>> {
        sig_pub: SigningPublicKey,
        box_pub: BoxPublicKey,
        box_shared: BoxSharedKey,
        intf: Box<dyn link::LinkInterfaceInner>,
        peer: PhantomData<P::Peer>,
    }

    impl<C: Core, P: PeerManager<C>> NewPeer<C, P> {
        #[inline]
        pub fn new(
            sig_pub: SigningPublicKey,
            box_pub: BoxPublicKey,
            box_shared: BoxSharedKey,
            intf: Box<dyn link::LinkInterfaceInner>,
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

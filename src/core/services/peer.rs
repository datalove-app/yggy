use crate::core::{types::*, Core};
use xactor::*;

/// Represents peer
pub trait PeerManager<C: Core, P: Peer<C, Self>>
where
    Self: Actor,
    Self: Handler<NewPeer<P>>,
    Self: Handler<Close>,
{
}

pub trait Peer<C: Core, P: PeerManager<C, Self>>
where
    Self: Actor,
{
}

/// Signals the creation of a new `Peer`.
#[derive(Debug)]
pub struct NewPeer<P> {
    signing_pub_key: SigningPublicKey,
    box_pub_key: BoxPublicKey,
    box_shared_key: BoxSharedKey,
    peer: std::marker::PhantomData<P>,
}
impl<P: Send + 'static> Message for NewPeer<P> {
    type Result = Addr<P>;
}

/// Signals the closing of a `Peer` connection.
#[xactor::message(result = "()")]
#[derive(Debug)]
pub struct Close;

//! Maintains open, E2EE connections to other nodes on the network.

use crate::{
    dev::*,
    interfaces::Conn,
    types::{BoxPublicKey, Handle},
};
use std::sync::Arc;

///
/// ? Handle<...>
#[async_trait::async_trait]
pub trait SessionManager<C: Core>: Sized {
    ///
    type Session: Session<C, Self>;

    fn reconfigure(&mut self);

    async fn session_by_handle(&self, handle: &Handle) -> Option<Addr<Self::Session>>;

    async fn session_by_pub_key(&self, key: &BoxPublicKey) -> Option<Addr<Self::Session>>;

    async fn create_session(
        self: Arc<Self>,
        their_key: BoxPublicKey,
    ) -> Result<Addr<Self::Session>, Error>;
}

/// Cryptographic agreements between two nodes that allow the exchange of
/// end-to-end encrypted traffic.
/// ? can be polled until completion, producing a Session
pub trait Session<C: Core, S: SessionManager<C>>: Sized
where
    Self: Actor,
{
}

pub mod messages {
    use super::*;

    // ///
    // #[derive(Debug)]
    // pub struct CreateSession {}

    // #[async_trait::async_trait]
    // impl xactor::Message for CreateSession {
    //     type Result = ();
    // }
}

// pub mod blank {
//     ///
//     #[derive(Debug)]
//     pub struct SessionPingPong {
//         /// The sender's permanent key.
//         sender_perm_key: BoxPublicKey,
//         /// Session key to use.
//         sender_session_key: BoxPublicKey,
//         /// Random number used to identify the session.
//         handle: Handle,
//         coords: Coords,
//         timestamp: Instant,
//         is_pong: bool,
//         mtu: MTU,
//     }
// }

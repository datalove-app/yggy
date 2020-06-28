use crate::{
    error::Error,
    interfaces::{Conn, Core},
    types::{BoxPublicKey, Handle},
};
use std::sync::Arc;
use xactor::{Actor, Addr};

///
/// ? Handle<...>
#[async_trait::async_trait]
pub trait SessionManager<C: Core>: Sized {
    ///
    type Session: Session<C, Self>;

    fn reconfigure(&mut self);

    fn session_by_handle(&self, handle: &Handle) -> Option<Addr<Self::Session>>;

    fn session_by_pub_key(&self, key: &BoxPublicKey) -> Option<Addr<Self::Session>>;

    async fn create_session(
        self: Arc<Self>,
        their_key: BoxPublicKey,
    ) -> Result<Addr<Self::Session>, Error>;
}

///
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

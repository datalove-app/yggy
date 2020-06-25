use crate::core_interfaces::{Conn, Core};
use xactor::Actor;

///
/// ? Handle<...>
pub trait SessionManager<C: Core>: Sized // where
//     Self: Actor,
{
    ///
    type Session: Session<C, Self>;

    fn reconfigure(&mut self);
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

    ///
    #[derive(Debug)]
    pub struct NewSession {}

    #[async_trait::async_trait]
    impl xactor::Message for NewSession {
        type Result = ();
    }
}

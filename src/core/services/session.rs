// use super::Listener;
use actix::prelude::*;

///
/// ? Handle<...>
pub trait SessionManager // where
//     Self: SystemService,
{
    /// Information about an ongoing Session.
    ///
    type Session: Session;

    // ///
    // type Listener: Listener;
}

///
/// ? can be polled until completion, producing a Session
pub trait Session // where
//     Self: Actor,
{
}

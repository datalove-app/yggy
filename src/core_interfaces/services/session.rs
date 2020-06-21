use crate::core_interfaces::{Conn, Core};
use xactor::Actor;

///
/// ? Handle<...>
pub trait SessionManager<C: Core>: Sized
where
    Self: Actor,
{
    fn reconfigure(&mut self);
}

///
/// ? can be polled until completion, producing a Session
pub trait Session<C: Core, S: SessionManager<C>>: Sized
where
    Self: Actor,
{
}

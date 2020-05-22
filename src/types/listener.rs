use super::Conn;
use actix::prelude::*;

/// Produces a stream of `Conn`s.
///
/// Provided by core
///
/// ? Handle<...>
pub trait Listener<C: Conn> // where
//     Self: ActorStream<Actor = Self, Item = C> + Actor,
{
}

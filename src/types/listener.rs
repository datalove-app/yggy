use super::Conn;
use actix::prelude::*;

/// Produces a stream of `Conn`s
///
/// ? Handle<...>
pub trait Listener<C: Conn> // where
//     Self: ActorStream<Actor = Self, Item = C> + Actor,
{
}

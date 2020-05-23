use super::Conn;
use actix::prelude::*;

/// Produces a stream of `Conn`s.
///
/// Provided by core
///
/// ? Handle<...>
pub trait Listener
// pub trait Listener<C: Core>
// where
//     Self: ActorStream<Actor = Self, Item = C> + Actor,
{
    /// The type of connection used to produce
    type Conn: Conn;
}

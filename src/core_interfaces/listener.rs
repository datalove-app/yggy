use super::{Conn, Core};
use xactor::Actor;

/// Produces a stream of `Conn`s.
///
/// Provided by core
///
/// ? Handle<...>
pub trait Listener<C: Core>
where
    Self: Actor,
{
}

use crate::core::Core;
use xactor::*;

///
/// TODO
/// Seems to handle traffic from addresses listed in `ListenAddresses`.
pub trait Link<C: Core, L: LinkInterface<C, Self>>
where
    Self: Actor,
{
}

///
pub trait LinkInterface<C: Core, L: Link<C, Self>>
where
    Self: Actor,
{
}

use crate::{
    core_interfaces::{Core, PeerInterface},
    core_types::wire,
};
use xactor::{Actor, StreamHandler};

/// Handles packets to/from self.
pub trait Router<C: Core>
where
    Self: Actor,
    Self: StreamHandler<wire::Traffic>,
    Self: StreamHandler<wire::ProtocolTraffic>,
{
    // type Interface: PeerInterface;

    fn reconfigure(&mut self);
}

pub mod messages {
    use super::*;
}

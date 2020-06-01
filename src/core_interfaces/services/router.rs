use crate::{core_interfaces::Core, core_types::wire};
use xactor::{Actor, StreamHandler};

///
///
pub trait Router<C: Core>
where
    Self: Actor,
    Self: StreamHandler<wire::Traffic>,
    Self: StreamHandler<wire::ProtocolTraffic>,
{
    // type IncomingTraffic:

    fn reconfigure(&mut self);
}

pub mod messages {
}

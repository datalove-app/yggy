#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "darwin.rs"]
mod device;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod device;

use crate::core_interfaces::{
    tun::{messages, Tun, TunConn as ITunConn},
    Core,
};
use xactor::{Actor, Context, Handler};

///
///
///
#[derive(Debug)]
pub struct TunAdapter;

// impl TunAdapter {
//     pub fn setup() {}
// }

impl<C: Core> Tun<C> for TunAdapter {
    type Conn = TunConn;
    type Device = device::TunDevice;
}

#[async_trait::async_trait]
impl Actor for TunAdapter {}

#[async_trait::async_trait]
impl Handler<messages::IncomingConnection> for TunAdapter {
    async fn handle(&mut self, ctx: &Context<Self>, msg: messages::IncomingConnection) {
        unimplemented!()
    }
}

///
///
///
#[derive(Debug)]
pub struct TunConn;

impl<C: Core> ITunConn<C> for TunConn {}

#[async_trait::async_trait]
impl Actor for TunConn {}

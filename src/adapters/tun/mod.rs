#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "darwin.rs"]
mod device;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod device;

use self::device::Device as TunDevice;
use crate::{
    core_interfaces::{
        tun::{messages, Tun, TunConn as ITunConn},
        Core,
    },
    core_types::{Address, Subnet},
};
use boringtun::noise::{Tunn, TunnResult};
use std::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use xactor::{Actor, Addr, Context, Handler};

///
#[derive(Debug)]
enum State {
    
}

///
#[derive(Debug)]
pub struct TunAdapter<C: Core> {
    state: State,

    ///
    core: Addr<C>,
    ///
    listener: Arc<C::Listener>,
    ///
    dialer: C::Dialer,
    ///
    conn_by_addr: HashMap<Address, Addr<<Self as Tun<C>>::Conn>>,
    ///
    conn_by_subnet: HashMap<Subnet, Addr<<Self as Tun<C>>::Conn>>,
    // ///
    // writer:
    // ///
    // reader:
    // ///
    // iface: <Self as Tun<C>>::Device,
}

impl<C: Core> TunAdapter<C> {
    #[inline]
    pub fn new(core: Addr<C>, dialer: C::Dialer, listener: Arc<C::Listener>) -> Self {
        Self {
            core,
            listener,
            dialer,
            conn_by_addr: HashMap::default(),
            conn_by_subnet: HashMap::default(),
            // writer
            // reader
            // iface: TunDevice
        }
    }
}

#[async_trait::async_trait]
impl<C: Core> Tun<C> for TunAdapter<C> {
    type Conn = TunConn<C>;
    type Device = TunDevice;
}

#[async_trait::async_trait]
impl<C: Core> Actor for TunAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

// #[async_trait::async_trait]
// impl Handler<messages::IncomingConnection> for TunAdapter {
//     async fn handle(&mut self, ctx: &Context<Self>, msg: messages::IncomingConnection) {
//         unimplemented!()
//     }
// }

///
///
///
pub struct TunConn<C: Core> {
    adapter: Addr<TunAdapter<C>>,
    conn: C::Conn,
    wg: Tunn,
}

impl<C: Core> fmt::Debug for TunConn<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<C: Core> ITunConn<C> for TunConn<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for TunConn<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

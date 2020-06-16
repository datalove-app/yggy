#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "darwin.rs"]
mod socket;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod socket;

use self::socket::{Socket as TUNSocket, TunReader, TunWriter};
use crate::{
    core_interfaces::{
        tun::{messages, Tun, TunSocket},
        Core,
    },
    core_types::{Address, Subnet, MTU},
    error::Error,
};
use boringtun::noise::{Tunn, TunnResult};
use futures::{io, prelude::*, task};
use std::{
    collections::HashMap,
    fmt,
    pin::Pin,
    sync::{Arc, Mutex},
};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};

///
#[derive(Debug)]
pub struct TunAdapter<C: Core> {
    // ///
    // state: State,
    ///
    core: Addr<C>,
    // ///
    // /// once?
    // listener: Arc<C::Listener>,
    // ///
    // dialer: C::Dialer,
    // ///
    // conn_by_addr: HashMap<Address, Addr<<Self as Tun<C>>::Conn>>,
    // ///
    // conn_by_subnet: HashMap<Subnet, Addr<<Self as Tun<C>>::Conn>>,
    // ///
    // reader: Addr<<TUNSocket as TunSocket>::Reader>,
    ///
    writer: Addr<<TUNSocket as TunSocket>::Writer>,
}

impl<C: Core> TunAdapter<C> {
    #[inline]
    pub async fn new(
        core: Addr<C>,
        // dialer: C::Dialer,
        // listener: Arc<C::Listener>,
    ) -> Result<Self, Error> {
        let (reader, writer) = TUNSocket::open(MTU::default())?.split()?;
        let writer = writer.start().await;

        Ok(Self {
            core,
            // listener,
            // dialer,
            // conn_by_addr: HashMap::default(),
            // conn_by_subnet: HashMap::default(),
            // reader,
            writer,
        })
    }
}

#[async_trait::async_trait]
impl<C: Core> Tun<C> for TunAdapter<C> {
    // type Conn = TunConn<C>;
    type Socket = TUNSocket;
}

#[async_trait::async_trait]
impl<C: Core> Actor for TunAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

// #[async_trait::async_trait]
// impl<C: Core> Handler<messages::IncomingConnection> for TunAdapter<C> {
//     async fn handle(&mut self, ctx: &Context<Self>, msg: messages::IncomingConnection) {
//         unimplemented!()
//     }
// }

#[async_trait::async_trait]
impl<C: Core> StreamHandler<messages::Packet> for TunAdapter<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: messages::Packet) {
        unimplemented!()
    }
}

// ///
// ///
// ///
// pub struct TunConn<C: Core> {
//     ///
//     adapter: Addr<TunAdapter<C>>,

//     /// The yggdrasil connection.
//     conn: C::Conn,

//     /// Handles the underlying Wireguard crypto for the tunnel.
//     wg: Tunn,
// }

// impl<C: Core> fmt::Debug for TunConn<C> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         unimplemented!()
//     }
// }

// impl<C: Core> ITunConn<C> for TunConn<C> {}

// #[async_trait::async_trait]
// impl<C: Core> Actor for TunConn<C> {
//     async fn started(&mut self, ctx: &Context<Self>) {}
// }

impl Stream for TunReader {
    type Item = messages::Packet;

    ///
    /// TODO:
    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Option<Self::Item>> {
        unimplemented!()
    }
}

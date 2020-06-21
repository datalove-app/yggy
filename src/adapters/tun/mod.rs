#[cfg(any(target_os = "macos"))]
#[path = "macos.rs"]
mod interface;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod interface;

use self::interface::{Socket as TunSocket, TunReader, TunWriter};
use crate::{
    core_interfaces::{
        tun::{self, TunInterface},
        Conn, Core,
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

type ITunWriter = <TunSocket as TunInterface>::Writer;

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
    // reader: Addr<<TunSocket as TunInterface>::Reader>,
    ///
    writer: Addr<ITunWriter>,
}

impl<C: Core> TunAdapter<C> {
    #[inline]
    pub async fn new(
        core: Addr<C>,
        // dialer: C::Dialer,
        // listener: Arc<C::Listener>,
    ) -> Result<Self, Error> {
        let (reader, writer) = TunSocket::open()?.split()?;
        // TODO start for each thread?
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
impl<C: Core> tun::TunAdapter<C> for TunAdapter<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for TunAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

#[async_trait::async_trait]
impl<C: Core> Handler<tun::messages::IncomingConnection> for TunAdapter<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: tun::messages::IncomingConnection) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<tun::messages::Packet> for TunAdapter<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: tun::messages::Packet) {
        unimplemented!()
    }
}

///
///
#[derive(Debug)]
pub struct TunConn<C: Core> {
    ///
    adapter: Addr<TunAdapter<C>>,

    /// The yggdrasil connection.
    conn: C::Conn,
    // /// Handles the underlying Wireguard crypto for the tunnel.
    // wg: Tunn,
}

impl<C: Core> tun::TunConn<C, TunAdapter<C>> for TunConn<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for TunConn<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

impl Stream for TunReader {
    type Item = tun::messages::Packet;

    ///
    /// TODO:
    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Option<Self::Item>> {
        unimplemented!()
    }
}

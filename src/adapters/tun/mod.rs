#[cfg(any(target_os = "macos"))]
#[path = "macos.rs"]
mod interface;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod interface;

use self::interface::{Socket as TunSocket, TunReader, TunWriter};
use boringtun::noise::{Tunn, TunnResult};
use futures::{io, prelude::*, task};
use std::{
    collections::HashMap,
    fmt,
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};
use yggy_core::{
    error::Error,
    interfaces::{
        tun::{self, TunInterface},
        Conn, Core,
    },
    types::{Address, Subnet, MTU},
};

type ITunConn<C> = <TunAdapter<C> as tun::TunAdapter<C>>::Conn;
type ITunReader = <TunSocket as TunInterface>::Reader;
type ITunWriter = <TunSocket as TunInterface>::Writer;

///
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(120);

///
#[derive(Debug)]
pub struct TunAdapter<C: Core> {
    // ///
    // state: State,
    ///
    core: Addr<C>,

    // ///
    // dialer: C::Dialer,
    // ///
    // /// once?
    // listener: Arc<C::Listener>,
    ///
    conn_by_addr: HashMap<Address, Addr<ITunConn<C>>>,

    ///
    conn_by_subnet: HashMap<Subnet, Addr<ITunConn<C>>>,

    ///
    reader: ITunReader,

    ///
    writer: Addr<ITunWriter>,
}

impl<C: Core> TunAdapter<C> {
    #[inline]
    pub async fn start(
        core: Addr<C>,
        // dialer: C::Dialer,
        // listener: Arc<C::Listener>,
    ) -> Result<Addr<Self>, Error> {
        let (reader, writer) = TunSocket::open()?.split()?;

        // TODO start for each thread?
        let writer = writer.start().await?;

        let mut adapter = Self {
            core,
            // dialer,
            // listener,
            conn_by_addr: HashMap::default(),
            conn_by_subnet: HashMap::default(),
            reader,
            writer,
        };

        Ok(Actor::start(adapter).await?)
    }
}

#[async_trait::async_trait]
impl<C: Core> tun::TunAdapter<C> for TunAdapter<C> {
    type Conn = TunConn<C>;
}

#[async_trait::async_trait]
impl<C: Core> Actor for TunAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        // TODO subscribe to reader
        // ctx.add_stream((&mut self).reader);

        unimplemented!()
    }
}

// #[async_trait::async_trait]
// impl<C: Core> Handler<tun::messages::IncomingConnection> for TunAdapter<C> {
//     async fn handle(&mut self, ctx: &Context<Self>, msg: tun::messages::IncomingConnection) {
//         unimplemented!()
//     }
// }

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
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

// impl Stream for TunReader {
//     type Item = tun::messages::Packet;

//     ///
//     /// TODO:
//     fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Option<Self::Item>> {
//         unimplemented!()
//     }
// }

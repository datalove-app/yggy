use crate::{
    error::Error,
    types::{primitives::*, Conn},
};
use actix::prelude::*;

// TODO? look at https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

/// Represents a running TUN interface.
///
/// flow:
///     ? (why) initialized with a core.Dialer and core.Listener
/// on startup:
///     init handler loop
///         for each ygg.Conn in Listener.await
///             wrap each in TunConn (driving adapter)
///             close existing ones (by asking Tun.has_conn)
///                 save it
///             ? yconn.subscribe(TunnConn._read).or_timeout()
///             packets will come from a session (crypto-boxed!)
///     start a tunReader (actor)
///
///     start the ckr
///
/// ???? is a Port
/// ? Handle<IncomingConnection>
///     ? spawns TunConn `for ygg.Conn in Listener.await`
///     ? stores `Addr<TunConn>` by remote address and subnet
pub trait Tun<C: Conn> {
    const IPV6_HEADER_LEN: u8 = 40;

    ///
    type Conn: TunConn<C>;

    fn name(&self) -> &str;

    fn mtu(&self) -> &MTU;

    fn start(&mut self);
}

///
/// is an actor that represents a connection (session?) with a remote peer
///     polling polls internal ygg.Conn
///         pulling from a readBuffer (created upon dialing)
///
/// created:
///     - upon dialing
///
/// ???? is a Port
///      ? Handle<...>
pub trait TunConn<C: Conn> // where
//     Self: ActorStream<Actor = Self, Item = Vec<u8>> + Actor,
{
}

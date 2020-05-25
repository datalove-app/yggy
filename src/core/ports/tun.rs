use crate::{
    core::{types::*, Conn, Core},
    error::Error,
};
use xactor::{Actor, StreamHandler};

// TODO? look at https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

/// Represents a running TUN interface.
///
/// ## What is TUN, what is Wireguard, and why do we use them?
/// TODO
///
/// flow:
///     ? (why) initialized with a core.Dialer and core.Listener
/// on startup:
///     init handler loop
///         for each yg.Conn in Listener.await
///             wrap each in TunConn (driving adapter)
///             close existing ones (by asking Tun.has_conn)
///                 save it
///             ? yconn.subscribe(TunnConn._read).or_timeout()
///                 in reality, yg.Conn drains an inner buffer
///                 calls
///             packets will come from a session (crypto-boxed!)
///
///     start a tunReader (actor)
///
///     start the ckr
///
/// ???? is a Port
/// ? Handle<IncomingConnection>
///     ? spawns TunConn `for yg.Conn in Listener.await`
///     ? stores `Addr<TunConn>` by remote address and subnet
pub trait Tun<C: Core>
where
    Self: Actor,
{
    const IPV6_HEADER_LEN: u8 = 40;

    // ///
    // type Conn: TunConn<C>;

    fn name(&self) -> &str;

    fn mtu(&self) -> &MTU;
}

///
/// is an actor that represents a connection (session?) with a remote peer
///     polling polls internal yg.Conn
///         pulling from a readBuffer (created upon dialing)
///
/// created:
///     - upon dialing
///
/// ???? is a Port
///      ? Handle<...>
pub trait TunConn<C: Conn>
where
    Self: Actor,
{
    // type Reader: Conn::
}

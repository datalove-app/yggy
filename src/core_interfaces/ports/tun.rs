use crate::{
    core_interfaces::{Conn, Core},
    core_types::*,
    error::Error,
};
use std::time::Duration;
use xactor::{Actor, Handler, Message, StreamHandler};

///
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(120);
// ///
// pub const OFFSET_BYTES: usize = 4;
// ///
// pub const HEADER_LENGTH: usize = 40;

// ? look at https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

/// Represents a running TUN interface.
///
/// ## What is TUN, what is Wireguard, and why do we use them?
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
///             packets go to a session (crypto-boxed!)
///
///     start a tunReader (actor)
///         waits for packet delivered by TUN device (iface)
///         Tun::_handle_packet (sends packet to TunConn)
///             finds cached TunConn from Tun
///                 or calls ygg.Dialer.Dial to create a ygg.Conn
///                 then wraps it as TunConn
///             calls TunConn.writefrom
///     start the ckr
///
/// ???? is a Port
/// ? Handle<IncomingConnection>
///     ? spawns TunConn `for yg.Conn in Listener.await`
///     ? stores `Addr<TunConn>` by remote address and subnet
pub trait Tun<C: Core>
where
    Self: Actor,
    Self: Handler<messages::IncomingConnection>,
{
    const IPV6_HEADER_LEN: u8 = 40;

    ///
    type Conn: TunConn<C>;

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
pub trait TunConn<C: Core>
where
    Self: Actor,
{
    // type Reader: Conn::
}

pub mod messages {
    #[xactor::message(result = "()")]
    #[derive(Debug)]
    pub struct IncomingConnection;

    // pub struct
}
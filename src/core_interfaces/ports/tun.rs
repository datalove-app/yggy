use crate::{
    core_interfaces::{Conn, Core},
    core_types::MTU,
    error::Error,
};
use futures::io::{AsyncRead, AsyncWrite};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use xactor::{Actor, Addr, Handler, Message, StreamHandler};

///
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(120);
// ///
// pub const OFFSET_BYTES: usize = 4;
// ///
// pub const HEADER_LENGTH: usize = 40;

/// Represents a running TUN interface.
///
/// ## What is TUN, what is Wireguard, and why do we use them?
///
/// flow:
///     ? (why) initialized with a core.Dialer and core.Listener
/// on startup:
///     init handler loop
///         for each yg.Conn in Listener.await
///             _wrap each in TunConn (driving adapter)
///             close existing ones (by asking Tun.has_conn)
///                 save it
///             ? yconn.subscribe(TunnConn._read).or_timeout()
///                 in reality, yg.Conn drains an inner buffer
///                     gives it to TunConn._read
///                 calls tunWriter.writeFrom
///                     calls iface.write
///
///     start a tunReader (actor)
///         waits for packet delivered by TUN device (iface)
///         tunAdapter._handle_packet (sends packet to TunConn)
///             finds cached TunConn from Tun
///                 or calls ygg.Dialer.Dial to create a ygg.Conn
///                 then wraps it as TunConn
///             calls TunConn.writefrom (._write)
///                 creates FlowKeyMessage
///                 ygg.Conn.WriteFrom
///                     then if packet too big:
///                         tunWriter.writeFrom(ICMP packet)
///
///     start the ckr
///
/// ???? is a Port
/// ? Handle<IncomingConnection>
///     ? spawns TunConn `for yg.Conn in Listener.await`
///     ? stores `Addr<TunConn>` by remote address and subnet
#[async_trait::async_trait]
pub trait Tun<C: Core>
where
    Self: Actor,
{
    const IPV6_HEADER_LEN: u8 = 40;

    // ///
    // type Conn: TunConn<C>;
    ///
    type Socket: TunSocket;
}

// ///
// /// is an actor that adapts an yg.Conn to an TUN interface connection w/ a remote peer
// ///     polling polls internal yg.Conn
// ///         pulling from a readBuffer (created upon dialing)
// ///
// /// created:
// ///     - upon dialing
// ///
// /// ???? is a Port
// ///      ? Handle<...>
// pub trait TunConn<C: Core>
// where
//     Self: Actor,
// {
//     // type Reader: Conn::
// }

pub mod messages {
    #[xactor::message(result = "()")]
    #[derive(Debug)]
    pub struct IncomingConnection;

    // pub struct
}

/// Represents the underlying, platform-specific TUN socket interface.
pub trait TunSocket: Sized {
    type Reader: AsyncRead;
    type Writer: AsyncWrite;

    // TODO: set interface name
    fn open(mtu: MTU) -> Result<Self, Error>;

    fn name(&self) -> &str;

    // fn mtu(&self) -> MTU;

    fn split(self) -> Result<(Self::Reader, Self::Writer), Error>;
}

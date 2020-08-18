//!

use crate::{dev::*, interfaces::Conn, types::MTU};
use std::sync::{Arc, Mutex};

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
pub trait TunAdapter<C: Core>
where
    Self: Actor,
{
    // const IPV6_HEADER_LEN: u8 = 40;

    type Conn: TunConn<C, Self>;
}

///
/// is an actor that adapts an yg.Conn to an TUN interface connection w/ a remote peer
///     polling polls internal yg.Conn
///         pulling from a readBuffer (created upon dialing)
///
/// created:
///     - upon dialing
///
/// ???? is a Port
///      ? Handle<...>
pub trait TunConn<C: Core, T: TunAdapter<C>>
where
    Self: Actor,
{
}

/// Represents the underlying, platform-specific TUN interface.
pub trait TunInterface: Sized {
    type Reader: AsyncRead;
    type Writer: Actor + AsyncWrite;

    // TODO: set interface name
    fn open() -> Result<Self, Error>;

    fn name(&self) -> &str;

    // fn mtu(&self) -> MTU;

    fn split(self) -> (Self::Reader, Self::Writer);
}

pub mod messages {
    #[xactor::message(result = "()")]
    #[derive(Debug)]
    pub struct IncomingConnection;

    // #[xactor::message(result = "()")]
    // #[derive(Debug)]
    // pub struct Packet;
}

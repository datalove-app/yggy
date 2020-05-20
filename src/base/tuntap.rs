use super::{types::*, Conn, Dialer, Error, Listener};
use actix::prelude::*;

// TODO? look at https://github.com/actix/actix/blob/master/examples/chat/src/main.rs

/// Represents a running TUN interface.
///
/// flow:
///     ?? (why) initialized with a core.Dialer and core.Listener
/// on startup:
///     init handler loop
///         subscribe to yconns from Listener
///         wrap each in TunConn
///             close existing ones
///             save it
///         yconn.SetReadCallback to have TunConn read bytes from buf (or timeout)
///             TODO:
///             packets will come from a session (crypto-boxed!)
///             TODO: ^^^
///     start a tunReader (actor)
///     start the ckr
///
///
/// ?? Handle<IncomingConnection> -> subscribed to stream from Listener, producing C -> TunConn
pub trait TunAdapter<C: Conn> {
    const IPV6_HEADER_LEN: u8 = 40;

    ///
    type Conn: TunConn<C>;

    fn name(&self) -> &str;

    fn mtu(&self) -> &MTU;

    fn start(&mut self);
}

///
/// TODO? is an actor that represents a connection (session?) with a remote peer
/// created:
///     - upon dialing
///     ?? Handle<...>
pub trait TunConn<C: Conn> {}
// where
//     Self: ActorStream {}

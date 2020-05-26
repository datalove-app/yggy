//!

// mod admin;
mod conn;
mod dialer;
mod listener;
mod ports;
pub mod services;
pub mod types;

// #[doc(inline)]
// pub use admin::Admin;
#[doc(inline)]
pub use conn::Conn;
#[doc(inline)]
pub use dialer::Dialer;
#[doc(inline)]
pub use listener::Listener;
#[doc(inline)]
pub use ports::{Link, Multicast, Tun};

use self::types::{BoxKeypair, SigningKeypair};
use crate::error::Error;
use async_std::prelude::Future;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use xactor::Actor;

///
/// TODO
pub trait Core
where
    Self: Actor,
{
    type Conn: Conn;
    type Dialer: Dialer<Self>;
    type Listener: Listener<Self>;
}

///
/// TODO: follow startup from yggdrasil-go
///     - init random or from stdin or file config
///     ...
///     - init a logger
///     - start node.core - starts DHT, router, switch, other core components
///         inits core (structs; peers, router, switchtable)
///         inits link
///         inits switchtable
///         inits router
///         starts peer loop, to call each peer
///             link.call tcp or socks
///     - register session firewall
///     - init AdminSocket, setup admin handlers
///     - init Multicast, then connect admin handlers
///     - init tuntap interface
///         init Listener and Dialer (n.core.Conn{Listen,Dialer})
///         init TunAdapter with l,d, then start
///         setup admin handlers
///     - log info, catch interrupts for quit/reload config
///     -
///
/// ?? Handle<...>
#[async_trait]
pub trait Node<C, /* A: Admin */ L, M, T>
where
    C: Core,
    // A: Admin,
    L: Link<C>,
    M: Multicast<C>,
    T: Tun<C>,
    Self: Sized,
{
    type Config;

    ///
    async fn from_config<F>(load_config: F) -> Result<Self, Error>
    where
        F: Future<Output = Self::Config>;

    /// Augments/replaces
    /// TODO
    async fn with_signing_keys<F>(self, load_kp: F) -> Result<Self, Error>
    where
        F: Future<Output = SigningKeypair>;
    async fn with_box_keys<F>(self, load_kp: F) -> Result<Self, Error>
    where
        F: Future<Output = BoxKeypair>;
}

// #[async_trait]
// pub trait NodeAPI: Node {
//     /// TODO
//     async fn get_peers(self: Arc<Self>) -> Result<(), Error>;

//     /// TODO
//     async fn get_switch_peers(self: Arc<Self>) -> Result<(), Error>;

//     /// TODO
//     async fn get_dht(self: Arc<Self>) -> Result<(), Error>;

//     /// TODO
//     async fn get_switch_queues(self: Arc<Self>) -> Result<(), Error>;

//     /// TODO
//     async fn get_sessions(self: Arc<Self>) -> Result<(), Error>;

//     // /// TODO
//     // async fn get_dialer(self: Arc<Self>) -> Result<(), Error>;

//     // /// TODO
//     // async fn get_listener(self: Arc<Self>) -> Result<(), Error>;

//     /// TODO
//     async fn get_tcp_listener(self: Arc<Self>) -> Result<(), Error>;
// }

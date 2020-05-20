pub mod config;
mod conn;
mod dialer;
pub mod error;
mod listener;
mod multicast;
mod router;
mod search;
mod session;
mod tuntap;
pub mod types;

pub use conn::Conn;
pub use dialer::Dialer;
pub use error::*;
pub use listener::Listener;
pub use multicast::Multicast;
pub use router::Router;
pub use search::{Search, SearchManager};
pub use session::Session;
pub use tuntap::TunAdapter;

use self::{
    config::Config,
    error::{ConfigError, Error},
    types::{BoxKeypair, SigningKeypair},
};
use actix::prelude::*;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

///
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
pub trait Node<C: Conn, T: TunAdapter<C>>
where
    Self: SystemService,
{
    type Dialer: Dialer<C>;

    ///
    fn from_config(config: Config) -> Result<Self, Error>;

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

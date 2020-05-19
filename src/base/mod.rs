pub mod config;
mod dialer;
pub mod error;
mod listener;
mod tuntap;
pub mod types;

pub use dialer::Dialer;
pub use error::*;
pub use listener::Listener;
pub use tuntap::TunAdapter;

use self::{
    config::Config,
    error::{ConfigError, Error},
    types::{BoxKeypair, SigningKeypair},
};
use actix::{Actor, SystemService};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

///
///
/// startup from yggdrasil-go
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
#[async_trait]
pub trait Node</* AdminSocket, Multicast */ TunAdapter>
where
    Self: SystemService,
{
    ///
    fn from_config(config: Config) -> Result<Self, Error>;

    /// Augments/replaces
    /// TODO
    fn with_signing_keys(self, kp: SigningKeypair) -> Self;
    fn with_box_keys(self, kp: BoxKeypair) -> Self;
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

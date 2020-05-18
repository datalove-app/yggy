pub mod config;
pub mod crypto;
mod dialer;
pub mod error;
mod listener;
mod tuntap;
pub mod types;

pub use dialer::Dialer;
pub use listener::Listener;
pub use tuntap::TunAdapter;

use self::{config::Config, error::Error};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Node {
    type State;

    ///
    async fn start(&mut self, config: Config) -> Result<Self::State, Error>;

    ///
    fn update_config(self: Arc<Self>, config: Config);
}

#[async_trait]
pub trait NodeAPI: Node {
    /// TODO
    async fn get_peers(self: Arc<Self>) -> Result<(), Error>;

    /// TODO
    async fn get_switch_peers(self: Arc<Self>) -> Result<(), Error>;

    /// TODO
    async fn get_dht(self: Arc<Self>) -> Result<(), Error>;

    /// TODO
    async fn get_switch_queues(self: Arc<Self>) -> Result<(), Error>;

    /// TODO
    async fn get_sessions(self: Arc<Self>) -> Result<(), Error>;

    // /// TODO
    // async fn get_dialer(self: Arc<Self>) -> Result<(), Error>;

    // /// TODO
    // async fn get_listener(self: Arc<Self>) -> Result<(), Error>;

    /// TODO
    async fn get_tcp_listener(self: Arc<Self>) -> Result<(), Error>;
}

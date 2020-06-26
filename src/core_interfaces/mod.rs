//!

// mod admin;
mod conn;
mod dialer;
mod listener;
mod node;
mod ports;
mod services;

// #[doc(inline)]
// pub use admin::Admin;
#[doc(inline)]
pub use conn::Conn;
#[doc(inline)]
pub use dialer::Dialer;
#[doc(inline)]
pub use listener::Listener;
#[doc(inline)]
pub use node::Node;
#[doc(inline)]
pub use ports::*;
#[doc(inline)]
pub use services::*;

use crate::{
    core_types::{BoxKeypair, SigningKeypair},
    error::Error,
    Config,
};
use std::fmt::Debug;
use xactor::{Actor, Addr, Handler};

///
#[async_trait::async_trait]
pub trait Core: Debug
where
    Self: Actor,
    Self: Handler<messages::GetConfig>,
{
    ///
    type Conn: Conn;

    ///
    type Dialer: Dialer<Self>;

    ///
    type Listener: Listener<Self>;

    ///
    type Router: Router<Self>;

    ///
    type PeerManager: PeerManager<Self>;

    ///
    async fn current_config(core: &mut Addr<Self>) -> Result<Config, Error> {
        Ok(core.call(messages::GetConfig).await?)
    }

    ///
    async fn dialer(core: &mut Addr<Self>) -> Result<Addr<Self::Listener>, Error>;

    ///
    async fn listener(core: &mut Addr<Self>) -> Result<Addr<Self::Listener>, Error>;

    ///
    async fn router(core: &mut Addr<Self>) -> Result<Addr<Self::Router>, Error>;

    ///
    async fn peer_manager(core: &mut Addr<Self>) -> Result<Addr<Self::PeerManager>, Error>;
}

pub mod messages {
    use crate::Config;

    #[xactor::message(result = "Config")]
    pub struct GetConfig;

    #[xactor::message(result = "Config")]
    pub struct Reconfigure;
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

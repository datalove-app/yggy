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

use crate::{error::Error, Config};
use std::{fmt::Debug, sync::Arc};
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
    type PeerManager: PeerManager<Self>;

    ///
    type Router: Router<Self>;

    ///
    type Switch: Switch<Self>;

    ///
    async fn current_config(core: &mut Addr<Self>) -> Result<Arc<Config>, Error> {
        Ok(core.call(messages::GetConfig).await?)
    }

    ///
    async fn dialer(core: &mut Addr<Self>) -> Result<Addr<Self::Listener>, Error>;

    ///
    async fn listener(core: &mut Addr<Self>) -> Result<Addr<Self::Listener>, Error>;

    ///
    async fn peer_manager(core: &mut Addr<Self>) -> Result<Addr<Self::PeerManager>, Error>;

    ///
    async fn router(core: &mut Addr<Self>) -> Result<Addr<Self::Router>, Error>;

    ///
    async fn switch(core: &mut Addr<Self>) -> Result<Addr<Self::Switch>, Error>;
}

pub mod messages {
    use crate::Config;
    use std::sync::Arc;

    #[xactor::message(result = "Arc<Config>")]
    pub struct GetConfig;

    #[xactor::message(result = "Arc<Config>")]
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

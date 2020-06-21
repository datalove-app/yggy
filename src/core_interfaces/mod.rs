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
};
use xactor::Actor;

///
/// TODO <D: DHT L: LinkManager, P: PeerManager, R: Router, Se: SearchManager, Ss: SessionManager>
pub trait Core
where
    Self: Actor,
{
    ///
    type Config;

    ///
    type Conn: Conn;

    ///
    type Dialer: Dialer<Self>;

    ///
    type Listener: Listener<Self>;

    ///
    type PeerManager: PeerManager<Self>;
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

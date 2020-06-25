use super::Router;
use crate::{
    core_interfaces::{router::session, Core},
    core_types::{AllowedEncryptionPublicKeys, BoxPublicKey, BoxSharedKey, Handle, MTU},
    error::Error,
};
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};

///
#[derive(Debug)]
pub struct SessionManager<C: Core> {
    ///
    core: Addr<C>,

    ///
    router: Addr<C::Router>,

    ///
    listener: Addr<C::Listener>,

    ///
    allowed_peer_keys: AllowedEncryptionPublicKeys,

    // ///
    // last_cleanup: SystemTime,
    ///
    max_mtu: MTU,

    ///
    sessions: HashMap<Handle, Addr<Session<C>>>,

    ///
    shared_keys: HashMap<BoxPublicKey, BoxSharedKey>,

    ///
    handles: HashMap<BoxPublicKey, Handle>,
}

impl<C: Core> SessionManager<C> {
    #[inline]
    pub async fn new(mut core: Addr<C>) -> Result<Self, Error> {
        let config = C::current_config(&mut core).await?;
        let listener = C::listener(&mut core).await?;
        let router = C::router(&mut core).await?;
        Ok(Self {
            core,
            listener,
            router,
            allowed_peer_keys: config.allowed_peer_keys,
            // last_cleanup: SystemTime::now(),
            max_mtu: config.interface_max_mtu,
            sessions: Default::default(),
            shared_keys: Default::default(),
            handles: Default::default(),
        })
    }
}

impl<C: Core> session::SessionManager<C> for SessionManager<C> {
    type Session = Session<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

// #[async_trait::async_trait]
// impl<C: Core> Actor for SessionManager<C> {
//     async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
//         unimplemented!()
//     }
// }

///
#[derive(Debug)]
pub struct Session<C: Core> {
    ///
    core: Addr<C>,

    ///
    session_manager: Arc<SessionManager<C>>,
}

impl<C: Core> session::Session<C, SessionManager<C>> for Session<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Session<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

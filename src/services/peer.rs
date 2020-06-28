use yggy_core::{dev::*, interfaces::peer, types};

///
#[derive(Debug)]
pub struct PeerManager<C: Core> {
    ///
    core: Addr<C>,
}

impl<C: Core> PeerManager<C> {
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        let mut adapter = Self { core };

        Ok(Actor::start(adapter).await?)
    }
}

impl<C: Core> peer::PeerManager<C> for PeerManager<C> {
    type Peer = Peer<C>;
}

#[async_trait::async_trait]
impl<C: Core> Actor for PeerManager<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<peer::messages::NewPeer> for PeerManager<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: peer::messages::NewPeer) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<peer::messages::Close> for PeerManager<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: peer::messages::Close) {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Peer<C: Core> {
    core: Addr<C>,
    peer_manager: Addr<PeerManager<C>>,
}

impl<C: Core> peer::Peer<C, PeerManager<C>> for Peer<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Peer<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

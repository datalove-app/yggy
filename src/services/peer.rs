use std::collections::HashMap;
use yggy_core::{dev::*, interfaces::peer, types::SwitchPort};

/// Represents peers with active connections.
///
/// Incoming packets are passed to the corresponding peer, which:
///     - in most cases, passes the packet to the handler for another peer
///     - else, the link protocol traffic is used to build the spanning tree,
/// by first checking the signatures and then passing the message along to the
/// switch.
#[derive(Debug)]
pub struct PeerManager<C: Core> {
    core: Addr<C>,
    ports: HashMap<SwitchPort, Addr<Peer<C>>>,
    // lookup_table
}

impl<C: Core> PeerManager<C> {
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        let mut adapter = Self {
            core,
            ports: Default::default(),
        };

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

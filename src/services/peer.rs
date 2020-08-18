use crate::{adapters::Link, services::Router};
use std::{collections::HashMap, sync::Arc, time::Instant};
use yggy_core::{
    dev::*,
    interfaces::{
        link,
        peer::{self, messages},
        router, switch,
    },
    types::{BoxPublicKey, BoxSharedKey, PeerURI, SigningPublicKey, SwitchPort},
};

type ISwitch<C> = <C as Core>::Switch;
type ILookupTable<C> = <ISwitch<C> as switch::Switch<C>>::LookupTable;

type PortsMap<C> = HashMap<SwitchPort, Addr<Peer<C>>>;

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
    ports: Arc<PortsMap<C>>,
    lookup_table: ILookupTable<C>,
}

impl<C: Core> PeerManager<C> {
    #[inline]
    pub async fn start(mut core: Addr<C>) -> Result<Addr<Self>, Error> {
        let mut switch = C::switch(&mut core).await?;
        let lookup_table = <ISwitch<C> as switch::Switch<C>>::get_lookup_table(&mut switch).await?;

        let mut adapter = Self {
            core,
            ports: Default::default(),
            lookup_table,
        };

        Ok(Actor::start(adapter).await?)
    }
}

#[async_trait::async_trait]
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
impl<C: Core> Handler<messages::NewPeer<C, Self>> for PeerManager<C> {
    async fn handle(
        &mut self,
        ctx: &Context<Self>,
        msg: messages::NewPeer<C, Self>,
    ) -> Result<Addr<<Self as peer::PeerManager<C>>::Peer>, Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<messages::ClosePeer> for PeerManager<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: messages::ClosePeer) {
        unimplemented!()
    }
}

/// TODO
#[derive(Debug)]
pub struct Peer<C: Core> {
    core: Addr<C>,
    peers: Addr<PeerManager<C>>,
    ports: Arc<PortsMap<C>>,

    // peer info
    port: SwitchPort,
    endpoint: PeerURI,
    sig: SigningPublicKey,
    r#box: BoxPublicKey,
    shared: BoxSharedKey,
    link_shared: BoxSharedKey,
    first_seen: Instant,
    intf: Box<dyn link::LinkInterfaceInner>,
    // peer stats
    // idle: bool,
    // dropping: bool,
}

impl<C: Core> Peer<C> {
    // pub async fn start(mut core: Addr<C>) -> Result<Addr<Self>, Error> {
    //     unimplemented!()
    // }
}

impl<C: Core> peer::Peer<C, PeerManager<C>> for Peer<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Peer<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<messages::HandlePacket> for Peer<C> {
    async fn handle(
        &mut self,
        ctx: &Context<Self>,
        msg: messages::HandlePacket,
    ) -> Result<usize, Error> {
        unimplemented!()
    }
}

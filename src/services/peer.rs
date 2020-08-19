use crate::{adapters::Link, services::Router};
use futures_locks::RwLock;
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

/// Sorted map of all
#[derive(Debug)]
struct PortsMap<C: Core>(RwLock<HashMap<SwitchPort, Addr<Peer<C>>>>);

impl<C: Core> PortsMap<C> {
    async fn insert(&self, peer: Addr<Peer<C>>) -> SwitchPort {
        let mut map = self.0.write().await;
        let mut i: SwitchPort = 0u64.into();
        loop {
            if !(*map).contains_key(&i) {
                map.insert(i, peer);
                break;
            }
            i += 1.into();
        }
        i
    }

    // async fn remove_peer(&self, port: SwitchPort) -> bool {
    //     let mut map = self.0.write().await;

    // }
}

impl<C: Core> Clone for PortsMap<C> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<C: Core> Default for PortsMap<C> {
    #[inline]
    fn default() -> Self {
        Self(RwLock::new(Default::default()))
    }
}

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
    ports: PortsMap<C>,
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
        let peer = Peer::start(self.core.clone(), ctx.address(), self.ports.clone(), msg).await?;
        self.ports.insert(peer.clone()).await;
        Ok(peer)
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
    peer_manager: Addr<PeerManager<C>>,
    ports: PortsMap<C>,

    // peer info
    port: SwitchPort,
    // endpoint: PeerURI,
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
    async fn start(
        mut core: Addr<C>,
        peer_manager: Addr<PeerManager<C>>,
        ports: PortsMap<C>,
        msg: messages::NewPeer<C, PeerManager<C>>,
    ) -> Result<Addr<Self>, Error> {
        let config = C::current_config(&mut core).await?;
        let shared = config.encryption_private_key.shared_key(&msg.box_pub);

        let peer = Peer {
            core,
            peer_manager,
            ports,
            port: Default::default(),
            // endpoint: Default::default(),
            sig: msg.sig_pub,
            r#box: msg.box_pub,
            shared,
            link_shared: msg.link_shared,
            first_seen: Instant::now(),
            intf: msg.intf,
        };

        Ok(Actor::start(peer).await?)
    }
}

impl<C: Core> peer::Peer<C, PeerManager<C>> for Peer<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Peer<C> {}

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

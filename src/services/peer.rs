use crate::{adapters::Link, services::Router};
use std::{collections::HashMap, sync::Arc};
use syllogism::IsNot;
use yggy_core::{
    dev::*,
    interfaces::{
        link,
        peer::{self, messages, IntoPeerInterface},
        router, switch,
    },
    types::{BoxPublicKey, BoxSharedKey, PeerURI, SigningPublicKey, SwitchPort},
};

type ISwitch<C> = <C as Core>::Switch;
type ILookupTable<C> = <ISwitch<C> as switch::Switch<C>>::LookupTable;

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
    ports: Arc<HashMap<SwitchPort, Addr<Peer<C>>>>,
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
    type PeerInterface = PeerInterface<C>;

    #[inline]
    async fn new_peer<L: IntoPeerInterface<C, Self>>(
        addr: &mut Addr<Self>,
        sig_pub: SigningPublicKey,
        box_pub: BoxPublicKey,
        link_shared: BoxSharedKey,
        intf_addr: Addr<L>,
    ) -> Result<Addr<Self::Peer>, Error>
// where
    //     Self::PeerInterface: From<Addr<L>>,
    {
        let msg = messages::NewPeer::<C, Self>::new(
            sig_pub,
            box_pub,
            link_shared,
            IntoPeerInterface::into(intf_addr),
        );
        Ok(addr.call(msg).await??)
    }
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

    intf: Option<PeerInterface<C>>,
    port: SwitchPort,
    endpoint: PeerURI,
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

// #[async_trait::async_trait]
// impl<C: Core> Handler<PeerInterface<C>> for Peer<C> {
//     async fn handle(&mut self, ctx: &Context<Self>, intf: PeerInterface<C>) {
//         self.intf.replace(intf);
//     }
// }

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

///
/// TODO
#[derive(Debug)]
pub enum PeerInterface<C: Core> {
    Link(Addr<Link<C>>),
    Router(Addr<Router<C>>),
}
// impl<C: Core> Message for PeerInterface<C> {
//     type Result = ();
// }

impl<C: Core> IntoPeerInterface<C, PeerManager<C>> for Link<C> {
    fn into(addr: Addr<Self>) -> PeerInterface<C> {
        PeerInterface::Link(addr)
    }
}

impl<C: Core> IntoPeerInterface<C, PeerManager<C>> for Router<C> {
    fn into(addr: Addr<Self>) -> PeerInterface<C> {
        PeerInterface::Router(addr)
    }
}

// impl<C: Core> IntoPeerInterface<C, PeerManager<C>> for L
// where
//     L: IsNot<Link<C>> + IsNot<Router<C>>,
// {
//     fn into(addr: Addr<Self>) -> P::PeerInterface {
//         panic!()
//     }
// }

impl<C: Core, L: link::LinkInterface> IsNot<L> for Link<C> {}
impl<C: Core, L: link::LinkInterface> IsNot<L> for Router<C> {}

// impl<C: Core> From<Addr<Link<C>>> for PeerInterface<C> {
//     fn from(addr: Addr<Link<C>>) -> Self {
//         Self::Link(addr)
//     }
// }

// impl<C: Core> From<Addr<Router<C>>> for PeerInterface<C> {
//     fn from(addr: Addr<Router<C>>) -> Self {
//         Self::Router(addr)
//     }
// }

// impl<C: Core, L: link::LinkInterface> From<Addr<L>> for PeerInterface<C>
// where
//     L: IsNot<Link<C>>,
//     L: IsNot<Router<C>>,
// {
//     fn from(addr: Addr<L>) -> Self {
//         panic!()
//     }
// }

// impl<C: Core, L: link::LinkInterface> From<Addr<L>> for PeerInterface<C>
// where
//     Addr<L>: Specialize<>

mod search;
mod session;

use std::sync::Arc;
use yggy_core::{
    dev::*,
    interfaces::{link, peer, router, switch},
    types::PeerURI,
};

type ILookupTable<C> = <ISwitch<C> as switch::Switch<C>>::LookupTable;
type IPeer<C> = <IPeerManager<C> as peer::PeerManager<C>>::Peer;
type IPeerManager<C> = <C as Core>::PeerManager;
type ISwitch<C> = <C as Core>::Switch;

///
#[derive(Debug)]
pub struct Router<C: Core> {
    core: Addr<C>,
    // dht
    searches: search::SearchManager<C>,
    sessions: session::SessionManager<C>,

    ///
    self_peer: Addr<IPeer<C>>,

    // ///
    // reader
    // ///
    // writer: RouterWriter<C>,
    lookup_table: Arc<ILookupTable<C>>,
}

impl<C: Core> router::Router<C> for Router<C> {
    type SearchManager = search::SearchManager<C>;
    type SessionManager = session::SessionManager<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

impl<C: Core> Router<C> {
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for Router<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<wire::Traffic> for Router<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: wire::Traffic) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<wire::ProtocolTraffic> for Router<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: wire::ProtocolTraffic) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> link::LinkInterface for Router<C> {
    type Inner = Addr<Self>;
    // type Reader = Unreadable;
    // type Writer = RouterWriter<C>;

    fn out<T: Wire>(intf: &mut Self::Inner, msg: T) {}

    fn link_out<T: Wire>(intf: &mut Self::Inner, msg: T) {}

    fn close(intf: &mut Self::Inner) {}

    fn name(&self) -> &str {
        "(self)"
    }

    fn local(&self) -> &PeerURI {
        unimplemented!()
    }

    fn remote(&self) -> &PeerURI {
        unimplemented!()
    }

    fn interface_type(&self) -> &str {
        "self"
    }
}

// ///
// /// TODO
// #[derive(Debug)]
// pub struct RouterWriter<C: Core> {
//     router: Addr<Router<C>>,
// }

// impl<C: Core> AsyncWrite for RouterWriter<C> {
//     fn poll_write(
//         mut self: Pin<&mut Self>,
//         cx: &mut task::Context,
//         buf: &[u8],
//     ) -> task::Poll<Result<usize, io::Error>> {
//         unimplemented!()
//     }

//     fn poll_flush(
//         mut self: Pin<&mut Self>,
//         cx: &mut task::Context,
//     ) -> task::Poll<Result<(), io::Error>> {
//         unimplemented!()
//     }

//     fn poll_close(
//         mut self: Pin<&mut Self>,
//         cx: &mut task::Context,
//     ) -> task::Poll<Result<(), io::Error>> {
//         unimplemented!()
//     }
// }

// ///
// #[derive(Debug)]
// pub struct Unreadable;

// impl AsyncRead for Unreadable {
//     fn poll_read(
//         mut self: Pin<&mut Self>,
//         cx: &mut task::Context,
//         buf: &mut [u8],
//     ) -> task::Poll<Result<usize, io::Error>> {
//         unreachable!()
//     }
// }

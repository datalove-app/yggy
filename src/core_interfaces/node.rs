use super::{Core, LinkAdapter, MulticastAdapter, TunAdapter};
use crate::{
    core_types::{BoxKeypair, SigningKeypair},
    error::Error,
};
use async_trait::async_trait;
use futures::Future;
use std::sync::{Arc, Mutex};
use xactor::Actor;

///
/// TODO: follow startup from yggdrasil-go
///     - init random or from stdin or file config
///     ...
///     - init a logger
///     - start node.core - starts DHT, router, switch, other core components
///         inits core (structs: peers, router, switchtable)
///         inits link
///         inits switchtable
///         inits router
///         starts peer loop, to call each peer
///             link.call tcp or socks
///     - register session firewall
///     - init AdminSocket, setup admin handlers
///     - init MulticastAdapter, then connect admin handlers
///     - init tuntap interface
///         init Listener and Dialer (n.core.Conn{Listen,Dialer})
///         init TunAdapter with l,d, then start
///         setup admin handlers
///     - log info, catch interrupts for quit/reload config
///     -
///
/// ?? Handle<...>
#[async_trait]
pub trait Node<C, /* A: Admin */ L, M, T>: Sized
where
    C: Core,
    // A: Admin,
    L: LinkAdapter<C>,
    M: MulticastAdapter<C>,
    T: TunAdapter<C>,
{
    // ///
    // async fn from_config<F>(load_config: F) -> Result<Self, Error>
    // where
    //     F: Future<Output = C::Config>;

    // /// Augments/replaces
    // /// TODO
    // async fn with_signing_keys<F>(self, load_kp: F) -> Result<Self, Error>
    // where
    //     F: Future<Output = SigningKeypair>;
    // async fn with_box_keys<F>(self, load_kp: F) -> Result<Self, Error>
    // where
    //     F: Future<Output = BoxKeypair>;
}

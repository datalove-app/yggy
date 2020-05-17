mod address;
mod ids;

pub use address::{Address, Subnet};
pub use ids::{NodeId, TreeId};

use crate::core::error::{ConfigError, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

///
pub type Peers = Vec<PeerURI>;

///
pub type InterfacePeers = HashMap<String, Peers>;

///
pub type ListenAddresses = Vec<PeerURI>;

///
pub type Ipv4Subnets = HashMap<(), ()>;

///
pub type Ipv6Subnets = HashMap<(), ()>;

///
#[derive(Debug, Deserialize, Serialize)]
pub enum PeerURI {
    TCP(SocketAddr),
    SOCKS(SocketAddr),
}

impl PeerURI {}

// TODO handle platform-specific opts
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl Default for PeerURI {
    fn default() -> Self {
        Self::TCP(SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 9001))
    }
}

impl FromStr for PeerURI {
    type Err = Error;

    #[inline]
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let prefix = match raw {
            _ if raw.starts_with("tcp://") => "tcp://",
            _ if raw.starts_with("socks://") => "socks://",
            _ => Err(ConfigError::InvalidPeerURI(raw.into()))?,
        };

        let addr: SocketAddr = raw
            .trim_start_matches(prefix)
            .parse()
            .map_err(|_| ConfigError::InvalidPeerURI(raw.into()))?;
        Ok(Self::TCP(addr))
    }
}

impl TryFrom<&str> for PeerURI {
    type Error = Error;

    #[inline]
    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        raw.parse()
    }
}

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InterfaceName {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "none")]
    None,
    Custom(String),
}

impl Default for InterfaceName {
    fn default() -> Self {
        Self::Auto
    }
}

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "u16")]
pub struct MTU(u16);

// TODO handle platform-specific
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl Default for MTU {
    fn default() -> Self {
        unimplemented!()
    }
}

// TODO handle platform-specific maximum
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl TryFrom<u16> for MTU {
    type Error = Error;

    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        if raw < 1280 {
            Err(ConfigError::InvalidMTU(raw).into())
        } else {
            Ok(Self(raw))
        }
    }
}

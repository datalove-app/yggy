mod address;
mod ids;

pub use address::{Address, Subnet};
pub use ids::{NodeId, TreeId};

use crate::error::{ConfigError, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

///
pub type Peers = Vec<PeerURI>;

///
pub type InterfacePeers = HashMap<String, PeerURI>;

///
pub type ListenAddresses = Vec<PeerURI>;

///
#[derive(Debug, Deserialize, Serialize)]
pub enum PeerURI {
    TCP(SocketAddr),
    SOCKS(SocketAddr),
}

impl PeerURI {
    // fn from
}

impl Default for PeerURI {
    fn default() -> Self {
        // TODO: #[cfg()]
        Self::TCP(SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 9001))
    }
}

impl TryFrom<&str> for PeerURI {
    type Error = Error;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
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

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "u16")]
pub struct MTU(u16);

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

use super::{BoxPublicKey, SigningPublicKey};
use crate::base::error::{ConfigError, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    time::Duration,
};

///
pub type Peers = Vec<PeerURI>;

///
pub type InterfacePeers = HashMap<String, Peers>;

#[derive(Debug, Default)]
pub struct Peer {
    pub_sign_key: SigningPublicKey,
    pub_box_key: BoxPublicKey,
    endpoint: PeerURI, // TODO protocol + endpoint + port
    bytes_sent: u64,
    bytes_recv: u64,
    uptime: Duration,
}

///
#[derive(Debug, Deserialize, Serialize)]
pub enum PeerURI {
    TCP(SocketAddr),
    SOCKS(SocketAddr, SocketAddr),
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

use super::{BoxPublicKey, SigningPublicKey, SwitchLocator, SwitchMessage, SwitchPort};
use crate::error::{ConfigError, Error};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{Duration, Instant},
};

///
/// TODO
type SourceInterface = String;

///
/// TODO feature-flag public/bootstrap nodes
#[derive(Clone, Debug, Deserialize, Default, Eq, Serialize, PartialEq)]
#[serde(transparent)]
pub struct PeerURIs(Vec<PeerURI>);

///
pub type InterfacePeerURIs = HashMap<SourceInterface, PeerURIs>;

///
/// TODO ensure untagged
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PeerURI {
    TCP(SocketAddr),
    // TODO SOCKS(SocketAddr, SocketAddr),
}

impl PeerURI {
    pub fn default_admin() -> Option<Self> {
        Some(Self::TCP(SocketAddr::new(
            Ipv4Addr::new(127, 0, 0, 1).into(),
            9000,
        )))
    }

    // TODO handle platform-specific opts
    pub fn default_listen() -> Self {
        // #[cfg(any(target_os = "macos", target_os = "ios"))] and
        // #[cfg(target_os = "linux")]
        Self::default()
    }

    // pub fn endpoint(&self) ->
    // pub fn port(&self)
}

impl Default for PeerURI {
    fn default() -> Self {
        Self::TCP(SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0))
    }
}

impl TryFrom<&str> for PeerURI {
    type Error = Error;

    #[inline]
    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        raw.parse()
    }
}

impl FromStr for PeerURI {
    type Err = Error;

    #[inline]
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            _ if raw.starts_with("tcp://") => {
                let addr = raw
                    .trim_start_matches("tcp://")
                    .parse()
                    .map_err(ConfigError::InvalidPeerURI)?;
                Ok(Self::TCP(addr))
            }
            // TODO: _ if raw.starts_with("socks://") => {
            //     let (addr1, addr2): (&str, &str) = raw
            //         .trim_start_matches("socks://")
            //         .split("/")
            //         .take(2)
            //         .collect_tuple()
            //         .ok_or_else(|| ConfigError::UnknownPeerURI(raw.into()))?;

            //     let addr1 = addr1.parse().map_err(ConfigError::InvalidPeerURI)?;
            //     let addr2 = addr2.parse().map_err(ConfigError::InvalidPeerURI)?;
            //     Ok(Self::SOCKS(addr1, addr2))
            // }
            _ => Err(ConfigError::UnknownPeerURI(raw.into()))?,
        }
    }
}

///
#[derive(Debug, Default)]
pub struct Peer {
    pub_sign_key: SigningPublicKey,
    pub_box_key: BoxPublicKey,
    endpoint: PeerURI, // TODO protocol + endpoint + port
    uptime: Duration,
    bytes_sent: u64,
    bytes_recv: u64,
}

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerInfo {
    key: SigningPublicKey,
    locator: SwitchLocator,
    port: SwitchPort,
    degree: u64,
    last_seen: Instant,
    // msg: SwitchMessage,
    is_blocked: bool,
}

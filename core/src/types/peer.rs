use crate::dev::*;
use itertools::Itertools;
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{Duration, Instant},
};

///
/// TODO feature-flag default public/bootstrap nodes
#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PeerURIs(Vec<PeerURI>);

impl PeerURIs {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PeerURI> {
        self.0.iter()
    }
}

///
pub type PeerURIsByInterface = HashMap<String, PeerURIs>;

///
/// TODO ensure serde untagged
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "&str")]
pub enum PeerURI {
    ///
    TCP(SocketAddr),

    // ///
    // UDP(SocketAddr),
    ///
    SOCKS(SocketAddr, SocketAddr),

    ///
    #[cfg(feature = "tor")]
    TOR(String, u16),
}

impl PeerURI {
    pub fn default_admin() -> Option<Self> {
        Some(Self::TCP(SocketAddr::new(
            Ipv4Addr::new(127, 0, 0, 1).into(),
            9000,
        )))
    }

    ///
    /// TODO handle platform-specific opts
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
                    .map_err(TypeError::InvalidPeerURI)?;
                Ok(Self::TCP(addr))
            }
            _ if raw.starts_with("socks://") => {
                let (addr1, addr2): (&str, &str) = raw
                    .trim_start_matches("socks://")
                    .split("/")
                    .take(2)
                    .collect_tuple()
                    .ok_or_else(|| TypeError::UnknownPeerURI(raw.into()))?;

                let addr1: SocketAddr = addr1.parse().map_err(TypeError::InvalidPeerURI)?;
                let addr2: SocketAddr = addr2.parse().map_err(TypeError::InvalidPeerURI)?;
                Ok(Self::SOCKS(addr1, addr2))
            }
            #[cfg(feature = "tor")]
            _ if raw.starts_with("tor:") => {
                const TOR_PORTS: &[u16] = &[
                    21, 22, 706, 1863, 5050, 5190, 5222, 5223, 6523, 6667, 6697, 8300,
                ];
                const TOR_PORT_ERR: &str = "must be a valid TOR LongLivedPort";

                let (domain, port): (&str, &str) = raw
                    .trim_start_matches("tor:")
                    .split(":")
                    .take(2)
                    .collect_tuple()
                    .ok_or_else(|| {
                        TypeError::InvalidTORPeerURI(
                            raw.into(),
                            "must be `tor:<domain>.onion:<port>`",
                        )
                    })?;

                if !domain.ends_with(".onion") {
                    Err(TypeError::InvalidTORPeerURI(
                        raw.into(),
                        "must be a valid `.onion` URL",
                    ))?;
                }

                let port: u16 = port
                    .parse()
                    .map_err(|_| TypeError::InvalidTORPeerURI(raw.into(), TOR_PORT_ERR))?;
                if !TOR_PORTS.contains(&port) {
                    Err(TypeError::InvalidTORPeerURI(raw.into(), TOR_PORT_ERR))?;
                }

                Ok(Self::TOR(domain.into(), port))
            }
            _ => Err(TypeError::UnknownPeerURI(raw.into()))?,
        }
    }
}

// ///
// #[derive(Debug, Default)]
// pub struct Peer {
//     pub_sign_key: SigningPublicKey,
//     pub_box_key: BoxPublicKey,
//     endpoint: PeerURI, // TODO protocol + endpoint + port
//     // socket:
//     uptime: Duration,
// }

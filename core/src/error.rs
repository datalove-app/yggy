use boringtun::noise::errors::WireGuardError;
use derive_more::From;
use std::net::Ipv6Addr;
use thiserror::Error;

///
#[derive(Debug, Error)]
pub enum Error {
    /// TODO:
    #[error("service initialization error: {0}")]
    Init(#[from] anyhow::Error),

    #[error("type error {0}")]
    Type(#[from] TypeError),

    #[error("{0}")]
    Conn(#[from] ConnError),

    #[error("wire error: {0}")]
    Wire(#[from] WireError),
}

/// Type errors.
#[derive(Debug, Error)]
pub enum TypeError {
    #[error("invalid MTU `{0}`: minimum acceptable is 1280")]
    InvalidMTU(u16),

    #[error("out of bounds IPv6 address `{0}`: must be within `200::/7`")]
    OutOfBoundsAddress(Ipv6Addr),

    #[error("invalid `NodeID`: {0}")]
    InvalidNodeID(String),

    #[error("invalid peer URI `{0}`")]
    InvalidPeerURI(#[from] std::net::AddrParseError),

    #[error("unknown peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    UnknownPeerURI(String),

    #[error("unable to parse private encryption key: `{0:?}`")]
    FailedPrivateKeyParsing(&'static str),

    #[error("unable to create shared encryption key: `{0:?}`")]
    FailedSharedKeyGeneration(WireGuardError),

    #[cfg(feature = "tor")]
    #[error("invalid TOR URI `{uri:?}`: {msg:?}")]
    InvalidTORPeerURI { uri: String, msg: &'static str },
}

/// Errors that occur when connecting to network interfaces and peers.
#[derive(Debug, Error)]
pub enum ConnError {
    #[error("interface error: {0}")]
    Interface(#[from] std::io::Error),

    #[error("session error: {0}")]
    Session(&'static str),
}

///
#[derive(Debug, Error)]
pub enum WireError {
    #[error("wire codec error: {0}")]
    Codec(&'static str),

    #[error("wire read error: {0}")]
    Read(std::io::Error),

    #[error("wire write error: {0}")]
    Write(std::io::Error),
}

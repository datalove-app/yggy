use derive_more::From;
use thiserror::Error;

///
#[derive(Debug, Error)]
pub enum Error {
    /// TODO:
    #[error("TODO")]
    Init,

    #[error("type error: {0}")]
    Type(#[from] TypeError),

    #[error("connection error: {0}")]
    Conn(#[from] ConnError),

    #[error("wire read error: {0}")]
    WireRead(std::io::Error),

    #[error("wire write error: {0}")]
    WireWrite(std::io::Error),
}

/// Type errors.
#[derive(Debug, Error)]
pub enum TypeError {
    #[error("invalid MTU `{0}`: minimum acceptable is 1280")]
    InvalidMTU(u16),

    #[error("invalid IPv6 peer address `{0}`: must be within `200::/7`")]
    OutOfBoundsAddress(ipnet::Ipv6Net),

    #[error("invalid `NodeID`: {0}")]
    InvalidNodeID(String),

    #[error("invalid peer URI `{0}`")]
    InvalidPeerURI(#[from] std::net::AddrParseError),

    #[error("unknown peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    UnknownPeerURI(String),
}

/// Errors that occur ...
/// TODO
/// impl Into<io::Error>
#[derive(Debug, Error)]
pub enum ConnError {}

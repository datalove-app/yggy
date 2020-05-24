use derive_more::From;
use thiserror::Error;

///
#[derive(Debug, Error)]
pub enum Error {
    /// TODO:
    #[error("TODO")]
    Init,

    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("connection error: {0}")]
    Conn(ConnError),

    #[error("wire read error: {0}")]
    WireRead(std::io::Error),

    #[error("wire write error: {0}")]
    WireWrite(std::io::Error),
}

/// Errors that occur during node configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid MTU `{0}`: minimum acceptable is 1280")]
    InvalidMTU(u16),

    #[error("invalid peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    InvalidPeerURI(#[from] std::net::AddrParseError),

    #[error("unknown peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    UnknownPeerURI(String),
}

/// Errors that occur ...
/// TODO
#[derive(Debug, Error)]
pub enum ConnError {}

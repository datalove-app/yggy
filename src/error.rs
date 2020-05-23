use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// TODO:
    #[error("TODO")]
    Init,

    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("wire read error: {0}")]
    WireReadError(std::io::Error),

    #[error("wire write error: {0}")]
    WireWriteError(std::io::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid MTU `{0}`: minimum acceptable is 1280")]
    InvalidMTU(u16),

    #[error("invalid peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    InvalidPeerURI(#[from] std::net::AddrParseError),

    #[error("unknown peer URI `{0}`: must be `tcp://...` or `socks://.../...`")]
    UnknownPeerURI(String),
}

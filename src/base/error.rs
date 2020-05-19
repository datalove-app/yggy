use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("TODO")]
    Init,

    #[error("configuration error: ")]
    Config(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid MTU `{0}`: minimum acceptable is 1280")]
    InvalidMTU(u16),

    #[error("invalid peer URI `{0}`: must be `tcp://` or `socks://`")]
    InvalidPeerURI(String),
}

use boringtun::crypto::x25519;
use derive_more::From;
use serde::{Deserialize, Serialize};

///
pub type SigningPublicKey = ed25519_dalek::PublicKey;

///
pub type SigningSecretKey = ed25519_dalek::SecretKey;

///
pub type EncryptionPublicKey = x25519::X25519PublicKey;

///
pub type EncryptionSecretKey = x25519::X25519SecretKey;

///
#[derive(Debug, Deserialize, From, Serialize)]
#[from(forward)]
#[serde(transparent)]
pub struct SigningKeypair(ed25519_dalek::Keypair);

///
#[derive(Debug)]
pub struct EncryptionKeypair {
    public: EncryptionPublicKey,
    private: EncryptionSecretKey,
}

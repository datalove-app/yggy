use boringtun::crypto::x25519;
use derive_more::From;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::hash::sha512;

/*
 * Protocol
 */

///
pub type SigningPublicKey = ed25519_dalek::PublicKey;

///
pub type SigningSecretKey = ed25519_dalek::SecretKey;

///
/// Used for protocol traffic.
#[derive(Debug, Deserialize, From, Serialize)]
#[from(forward)]
#[serde(transparent)]
pub struct SigningKeypair(ed25519_dalek::Keypair);

/// The identifier of an yggrdasil node in the root selection algorithm used to
/// construct the spanning tree.
/// It is the SHA-512 digest of the node's `SigningPublicKey`.
#[derive(Clone, Copy, Debug)]
pub struct TreeID(sha512::Digest);

impl Default for TreeID {
    fn default() -> Self {
        Self(sha512::Digest::from_slice([0; 64].as_ref()).expect("this should never fail"))
    }
}

impl From<&SigningPublicKey> for TreeID {
    #[inline]
    fn from(pub_key: &SigningPublicKey) -> Self {
        Self(sha512::hash(pub_key.as_bytes()))
    }
}

/*
 * Traffic
 */

///
#[derive(Debug, From)]
#[from(forward)]
pub struct BoxPublicKey(x25519::X25519PublicKey);

impl BoxPublicKey {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Default for BoxPublicKey {
    fn default() -> Self {
        Self::from([0; 32].as_ref())
    }
}

///
#[derive(Debug, From)]
#[from(forward)]
pub struct BoxSecretKey(x25519::X25519SecretKey);

impl BoxSecretKey {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

///
#[derive(Debug, From)]
#[from(forward)]
pub struct BoxSharedKey(x25519::X25519EphemeralKey);

///
pub type BoxNonce = [u8; 24];

///
/// Used for encapsulated IPv6 traffic.
#[derive(Debug)]
pub struct BoxKeypair {
    public: BoxPublicKey,
    secret: BoxSecretKey,
}

/// The identifier of an yggdrasil node in the DHT, used to derive IPv6
/// addresses and subnets.
/// It is the SHA-512 digest of the node's `BoxPublicKey`.
#[derive(Clone, Copy, Debug)]
pub struct NodeID(sha512::Digest);

impl NodeID {
    /// returns the number of bits set in a masked `NodeID`.
    pub fn prefix_len(&self) -> u8 {
        unimplemented!()
    }
}

impl Default for NodeID {
    fn default() -> Self {
        Self(sha512::Digest::from_slice([0; 64].as_ref()).expect("this should never fail"))
    }
}

impl From<&BoxPublicKey> for NodeID {
    #[inline]
    fn from(pub_key: &BoxPublicKey) -> Self {
        Self(sha512::hash(pub_key.as_bytes()))
    }
}

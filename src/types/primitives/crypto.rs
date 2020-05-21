use boringtun::crypto::x25519;
use derive_more::{From, FromStr};
use digest::{generic_array::GenericArray, Digest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::Sha512;

type InnerDigest = GenericArray<u8, <Sha512 as Digest>::OutputSize>;

/*
 * IDs
 */

/// The identifier of an yggdrasil node in the DHT, used to derive IPv6
/// addresses and subnets.
/// It is the SHA-512 digest of the node's `BoxPublicKey`.
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NodeID(InnerDigest);

impl NodeID {
    /// returns the number of bits set in a masked `NodeID`.
    pub fn prefix_len(&self) -> u8 {
        unimplemented!()
    }
}

impl From<&BoxPublicKey> for NodeID {
    #[inline]
    fn from(pub_key: &BoxPublicKey) -> Self {
        Self(Sha512::digest(pub_key.as_bytes()))
    }
}

/// The identifier of an yggrdasil node in the root selection algorithm used to
/// construct the spanning tree.
/// It is the SHA-512 digest of the node's `SigningPublicKey`.
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct TreeID(InnerDigest);

impl From<&SigningPublicKey> for TreeID {
    #[inline]
    fn from(pub_key: &SigningPublicKey) -> Self {
        Self(Sha512::digest(pub_key.as_bytes()))
    }
}

/*
 * Keys
 */

///
/// Used for protocol traffic.
#[derive(Debug, Deserialize, From, Serialize)]
#[from(forward)]
#[serde(transparent)]
pub struct SigningKeypair(ed25519_dalek::Keypair);

///
pub type SigningPublicKey = ed25519_dalek::PublicKey;

///
pub type SigningSecretKey = ed25519_dalek::SecretKey;

///
/// Used for encapsulated IPv6 traffic.
#[derive(Debug)]
pub struct BoxKeypair {
    public: BoxPublicKey,
    secret: BoxSecretKey,
}

///
pub type BoxNonce = [u8; 24];

///
#[derive(Debug, From, FromStr, Eq, Hash, PartialEq)]
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

impl Serialize for BoxPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_bytes())
    }
}

/// Tries to deserialize from bytes or hex or base64 string.
impl<'de> Deserialize<'de> for BoxPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor as DeVisitor};
        use std::str::FromStr;

        struct BoxPublicKeyVisitor;
        impl<'de> DeVisitor<'de> for BoxPublicKeyVisitor {
            type Value = BoxPublicKey;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an X25519 public encryption key")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                BoxPublicKey::from_str(v).map_err(Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(BoxPublicKey::from(v))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(&v)
            }
        }

        deserializer.deserialize_any(BoxPublicKeyVisitor)
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

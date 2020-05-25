use boringtun::crypto::x25519;
use derive_more::{From, FromStr};
use digest::{generic_array::GenericArray, Digest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::Sha512;
use std::cmp::Ordering;

/*
 * IDs
 */

/// The identifier of a node in the DHT, used to derive IPv6 addresses and
/// subnets, as well as route authenticated protocol traffic.
/// It is the SHA-512 digest of the node's [`BoxPublicKey`].
///
/// [`BoxPublicKey`]: ./struct.BoxPublicKey.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NodeID(InnerDigest);

impl NodeID {
    /// Returns the number of bits set in a masked `NodeID`.
    #[inline]
    pub fn prefix_len(&self) -> u8 {
        unimplemented!()
    }

    ///
    #[inline]
    pub fn mask(&self) {
        unimplemented!()
    }
}

impl From<&BoxPublicKey> for NodeID {
    #[inline]
    fn from(pub_key: &BoxPublicKey) -> Self {
        Self(Sha512::digest(pub_key.as_bytes()))
    }
}

/// The identifier of a node in the root selection algorithm used to construct
/// the spanning tree.
/// It is the SHA-512 digest of the node's [`SigningPublicKey`].
///
/// [`SigningPublicKey`]: ./struct.SigningPublicKey.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TreeID(InnerDigest);

impl From<&SigningPublicKey> for TreeID {
    #[inline]
    fn from(pub_key: &SigningPublicKey) -> Self {
        Self(Sha512::digest(pub_key.as_bytes()))
    }
}

///
pub(crate) type Handle = [u8; 8];

/*
 * Keys
 */

///
/// Used for protocol traffic.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SigningKeypair {
    public: SigningPublicKey,
    secret: SigningSecretKey,
}

///
pub type Signature = ed25519_dalek::Signature;

///
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, From, PartialEq, Serialize)]
#[from(forward)]
pub struct SigningPublicKey(ed25519_dalek::PublicKey);

impl SigningPublicKey {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// Computes the [`TreeID`] from the key's digest, then compares them.
/// This is used to compute new [spanning tree roots]().
///
/// [`TreeID`]: struct.TreeID
impl PartialOrd for SigningPublicKey {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            let id1 = TreeID::from(self);
            let id2 = TreeID::from(other);
            Some(id1.cmp(&id2))
        }
    }
}

/// Computes the [`TreeID`] from the key's digest, then compares them.
/// This is used to compute new [spanning tree roots]().
///
/// [`TreeID`]: struct.TreeID
impl Ord for SigningPublicKey {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other)
            .expect("comparing two `SigningPublicKey`s should never fail")
    }
}

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
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
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
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

///
#[derive(Debug, From)]
#[from(forward)]
pub struct BoxSharedKey(x25519::X25519EphemeralKey);

type InnerDigest = GenericArray<u8, <Sha512 as Digest>::OutputSize>;

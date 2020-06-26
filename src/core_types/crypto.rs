use crate::error::{Error, TypeError};
use bitvec::{
    order::Msb0,
    slice::{AsBits, BitSlice},
};
use boringtun::crypto::x25519;
use derive_more::{AsRef, From, FromStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{
    digest::{generic_array::GenericArray, Digest, FixedOutput},
    Sha512,
};
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
};

/*
 * IDs
 */

/// The identifier of a node in the DHT, used to derive IPv6 addresses and
/// subnets, as well as route authenticated protocol traffic.
/// It is the SHA-512 digest of the node's [`BoxPublicKey`].
///
/// [`BoxPublicKey`]: ./struct.BoxPublicKey.html
#[derive(AsRef, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[as_ref(forward)]
pub struct NodeID(InnerDigest);

impl NodeID {
    const BYTE_LENGTH: usize = 64;
    const MAX_PREFIX_LEN: u8 = 127;

    /// Returns the number of bits set in a masked `NodeID`.
    #[inline]
    pub fn prefix_len(&self) -> u8 {
        Self::leading_ones(self.as_ref()).expect("this should never fail")
    }

    ///
    #[inline]
    pub fn mask(&self) {
        unimplemented!()
    }

    #[inline]
    fn leading_ones(bytes: &[u8]) -> Option<u8> {
        let leading_ones: Option<u8> = bytes
            .bits::<Msb0>()
            .iter()
            .take_while(|b| **b)
            .count()
            .try_into()
            .ok();

        leading_ones.filter(|count| count <= &Self::MAX_PREFIX_LEN)
    }
}

/// TODO assert leading ones <= 127?
impl TryFrom<&BoxPublicKey> for NodeID {
    type Error = Error;

    #[inline]
    fn try_from(pub_key: &BoxPublicKey) -> Result<Self, Self::Error> {
        let digest = Sha512::digest(pub_key.as_bytes());
        let node_id = Self::leading_ones(&digest)
            .and(Some(Self(digest)))
            .ok_or_else(|| TypeError::InvalidNodeID("too many leading ones".into()))?;
        Ok(node_id)
    }
}

///
pub type NodeIDMask = InnerDigest;

/// The identifier of a node in the root selection algorithm used to construct
/// the spanning tree.
/// It is the SHA-512 digest of the node's [`SigningPublicKey`].
///
/// [`SigningPublicKey`]: ./struct.SigningPublicKey.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TreeID(InnerDigest);

impl TreeID {
    const BYTE_LENGTH: usize = 64;
}

impl From<&SigningPublicKey> for TreeID {
    #[inline]
    fn from(pub_key: &SigningPublicKey) -> Self {
        Self(Sha512::digest(pub_key.as_bytes()))
    }
}

///
#[derive(Clone, Copy, Debug, Default)]
pub struct Handle([u8; 8]);

impl Handle {
    ///
    /// TODO generate random handles
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

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

impl Clone for BoxPublicKey {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.as_bytes().into())
    }
}

// impl Default for BoxPublicKey {
//     #[inline]
//     fn default() -> Self {
//         Self::from([0; 32].as_ref())
//     }
// }

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
// #[serde(transparent)] TODO: try_from FromStr
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

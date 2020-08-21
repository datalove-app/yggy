use crate::dev::*;
use bitvec::{
    order::Msb0,
    slice::{AsBits, BitSlice},
};
use derive_more::{AsRef, Constructor, From, FromStr, Into};
use rand::{thread_rng, CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use sha2::{
    digest::{generic_array::GenericArray, Digest, FixedOutput},
    Sha512,
};
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    fmt, hash,
    str::FromStr,
    sync::{Arc, Mutex},
};
use zerocopy::{AsBytes, FromBytes};

// lazy_static! {
//     ///
//     ///
//     /// TODO is this too blocking?
//     static ref RNG: Mutex<ChaChaRng> = ChaChaRng::from_rng(thread_rng()).unwrap().into();
// }

type InnerDigest = GenericArray<u8, <Sha512 as Digest>::OutputSize>;

/*
 * IDs
 */

/// The identifier of a node in the DHT, used to derive IPv6 addresses and
/// subnets, as well as route authenticated protocol traffic.
/// It is the SHA-512 digest of the node's [`BoxPublicKey`].
///
/// [`BoxPublicKey`]: ./struct.BoxPublicKey.html
#[derive(AsRef, Clone, Debug, Default, Eq, Hash, PartialEq)]
#[as_ref(forward)]
pub struct NodeID(InnerDigest);

impl NodeID {
    ///
    const BYTE_LENGTH: usize = 64;

    /// Maximum number of leading ones in the `NodeID`, also known as the
    /// prefix length.
    const MAX_PREFIX_LENGTH: u8 = 127;

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
        let count: Option<u8> = bytes
            .bits::<Msb0>()
            .iter()
            .take_while(|b| **b)
            .count()
            .try_into()
            .ok();

        count.filter(|count| count <= &Self::MAX_PREFIX_LENGTH)
    }
}

/// Creates a `NodeID` from a `BoxPublicKey`, verifying it's prefix length.
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

/// A cryptographically-random handle, used to identify the [`Session`] to
/// which an incoming packet belongs.
///
/// [`Session`]: ../core_interfaces/services/router/session/trait.Session.html
#[derive(AsBytes, Clone, Copy, Debug, Eq, FromBytes, Hash, PartialEq)]
#[repr(C)]
pub struct Handle([u8; 8]);

impl Handle {
    /// Generates cryptographically-random session handles.
    #[inline]
    pub fn new() -> Self {
        let mut handle = [0u8; 8];
        (&mut thread_rng()).fill_bytes(&mut handle);
        Self(handle)
    }
}

/*
 * Keys
 */

// ///
// /// TODO docs
// /// Used for protocol traffic.
// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct SigningKeypair {
//     pub secret: SigningSecretKey,
//     pub public: SigningPublicKey,
// }

///
pub type Signature = ed25519_dalek::Signature;

///
/// TODO docs
#[derive(AsRef, Clone, Copy, Debug, Default, Deserialize, Eq, From, PartialEq, Serialize)]
#[from(forward)]
#[repr(transparent)]
#[serde(transparent)]
pub struct SigningPublicKey(ed25519_dalek::PublicKey);

impl SigningPublicKey {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl hash::Hash for SigningPublicKey {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
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
/// TODO docs
pub type SigningSecretKey = ed25519_dalek::SecretKey;

///
/// TODO docs
/// Used for encapsulated IPv6 traffic.
#[derive(Constructor, Debug)]
pub struct BoxKeypair {
    pub secret: BoxSecretKey,
    pub public: BoxPublicKey,
}

impl BoxKeypair {
    #[inline]
    pub fn random() -> Self {
        let secret = BoxSecretKey::random();
        let public = secret.public_key();
        Self { secret, public }
    }

    #[inline]
    pub fn shared_key(&self) -> BoxSharedKey {
        self.secret.shared_key(&self.public)
    }
}

///
/// TODO docs
#[derive(AsRef, Clone, Copy, Debug, Deserialize, From, Serialize)]
#[from(forward)]
#[repr(transparent)]
#[serde(transparent)]
pub struct BoxPublicKey(x25519_dalek::PublicKey);

impl BoxPublicKey {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Eq for BoxPublicKey {}
impl PartialEq for BoxPublicKey {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl hash::Hash for BoxPublicKey {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

// impl FromStr for BoxPublicKey {
//     type Err = Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let key = s.parse().map_err(TypeError::FailedBoxKeyParsing)?;
//         Ok(Self(Arc::new(key)))
//     }
// }

///
/// TODO docs
/// TODO should we repr?
#[derive(AsRef, Clone)]
#[repr(transparent)]
pub struct BoxSecretKey(x25519_dalek::StaticSecret);

impl BoxSecretKey {
    #[inline]
    pub fn random() -> Self {
        Self(x25519_dalek::StaticSecret::new(&mut thread_rng()))
    }

    #[inline]
    pub fn public_key(&self) -> BoxPublicKey {
        BoxPublicKey((&self.0).into())
    }

    #[inline]
    pub fn shared_key(&self, peer_public: &BoxPublicKey) -> BoxSharedKey {
        self.0.diffie_hellman(peer_public.as_ref()).into()
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

// TODO
impl From<&SigningSecretKey> for BoxSecretKey {
    fn from(sig_key: &SigningSecretKey) -> Self {
        unimplemented!()
    }
}

impl fmt::Debug for BoxSecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxSecretKey: {:?}", &self.to_bytes())
    }
}

// impl FromStr for BoxSecretKey {
//     type Err = Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let key = s.parse().map_err(TypeError::FailedBoxKeyParsing)?;
//         Ok(Self(Arc::new(key)))
//     }
// }

impl Serialize for BoxSecretKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0.to_bytes())
    }
}

/// Tries to deserialize from hex or base64 string.
/// TODO? from hex or base64 str
impl<'de> Deserialize<'de> for BoxSecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor as DeVisitor};
        use std::str::FromStr;

        struct BoxSecretKeyVisitor;
        impl<'de> DeVisitor<'de> for BoxSecretKeyVisitor {
            type Value = BoxSecretKey;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an X25519 secret encryption key")
            }

            // fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            // where
            //     E: Error,
            // {
            //     BoxSecretKey::from_str(v).map_err(Error::custom)
            // }

            // fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            // where
            //     E: Error,
            // {
            //     self.visit_str(&v)
            // }
        }

        deserializer.deserialize_any(BoxSecretKeyVisitor)
    }
}

///
#[derive(AsRef, From)]
#[from(forward)]
#[repr(transparent)]
pub struct BoxSharedKey(x25519_dalek::SharedSecret);

impl BoxSharedKey {
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for BoxSharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxSharedKey: {:?}", self.as_bytes())
    }
}

///
/// TODO docs
#[derive(AsBytes, Copy, Clone, Debug, Eq, FromBytes, PartialEq)]
#[repr(C)]
pub struct BoxNonce([u8; 24]);

impl BoxNonce {
    #[inline]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        loop {
            let mut nonce = [0u8; 24];
            (&mut rng).fill_bytes(&mut nonce);

            if nonce[0] != 0xff {
                return Self(nonce);
            }
        }
    }
}

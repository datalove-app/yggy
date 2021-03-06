use crate::{
    dev::*,
    types::{BoxPublicKey, SigningPublicKey},
};
use futures_codec::{Decoder, Encoder};
use std::cmp::Ordering;

/// Template metadata for the current yggdrasil protocl version.
pub const CURRENT_METADATA: Metadata = METADATA_V0_2;

/// Metadata for yggdrasil protocol version v0.2
pub const METADATA_V0_2: Metadata = Metadata {
    meta: ['m' as u8, 'e' as u8, 't' as u8, 'a' as u8],
    major_version: 0,
    minor_version: 2,
    keys: None,
};

/// Version-specific metadata exchanged at the start of a connection.
///
/// Always begins with the bytes `'meta'` and a wire-formatted `u64` major
/// version number.
/// The current version also includes a wire-formatted `u64` minor version
/// number, and the box/sig/link keys that need to be exchanged to establish
/// a connection.
#[derive(Clone, Debug)]
pub struct Metadata {
    meta: [u8; 4],
    major_version: u64,
    minor_version: u64,
    pub keys: Option<MetadataKeys>,
}

impl Metadata {
    #[inline]
    pub const fn new(r#box: BoxPublicKey, sig: SigningPublicKey, link: BoxPublicKey) -> Self {
        Self {
            keys: Some(MetadataKeys { r#box, sig, link }),
            ..CURRENT_METADATA
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetadataKeys {
    pub r#box: BoxPublicKey,
    pub sig: SigningPublicKey,
    pub link: BoxPublicKey,
}

impl Default for Metadata {
    fn default() -> Self {
        CURRENT_METADATA
    }
}

impl Eq for Metadata {}
impl PartialEq for Metadata {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta
            && self.major_version == other.major_version
            && self.minor_version == other.minor_version
    }
}

impl PartialOrd for Metadata {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major_version > other.major_version {
            Some(Ordering::Greater)
        } else if self.major_version == other.major_version
            && self.minor_version > other.minor_version
        {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for Metadata {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("comparing two `Metadata`s should never fail")
    }
}

impl Wire for Metadata {
    // const LENGTH: usize = 0;

    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

impl Wire for MetadataKeys {
    // const LENGTH: usize = 0;

    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

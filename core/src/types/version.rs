use super::crypto::{BoxPublicKey, SigningPublicKey};
use std::cmp::Ordering;

/// Template metadata for
pub const CURRENT_METADATA: Metadata = METADATA_V0_2;

/// Metadata for yggdrasil protocol version v0.2
pub const METADATA_V0_2: Metadata = Metadata {
    meta: ['m' as u8, 'e' as u8, 't' as u8, 'a' as u8],
    maj_version: 0,
    min_version: 2,
    keys: None,
};

/// Version-specific metadata exchanged at the start of a connection.
///
/// Always begins with the bytes `'meta'` and a wire-formatted `u64` major
/// version number.
/// The current version also includes a wire-formatted `u64` minor version
/// number, and the box/sig/link keys that need to be exchanged to establish
/// a `wg` connection.
#[derive(Debug)]
pub struct Metadata {
    meta: [u8; 4],
    maj_version: u64,
    min_version: u64,
    keys: Option<MetadataKeys>,
}

impl Eq for Metadata {}
impl PartialEq for Metadata {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta
            && self.maj_version == other.maj_version
            && self.min_version == other.min_version
    }
}

impl PartialOrd for Metadata {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl Ord for Metadata {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("comparing two `Metadata`s should never fail")
    }
}

#[derive(Debug)]
struct MetadataKeys {
    pub r#box: Option<BoxPublicKey>,
    pub sig: Option<SigningPublicKey>,
    pub link: Option<BoxPublicKey>,
}

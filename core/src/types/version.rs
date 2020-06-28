use std::cmp::Ordering;

///
#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {
    meta: [u8; 4],
    maj_version: u64,
    min_version: u64,
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
            .expect("comparing to `Metadata`s should never fail")
    }
}

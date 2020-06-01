use super::NodeID;
use std::time::{Duration, Instant};

/// The time after which we timeout a search (so it can be restarted).
pub const RETRY_TIMEOUT: Duration = Duration::from_secs(3);
pub const STEP_TIME: Duration = Duration::from_secs(1);
pub const MAX_RESULTS: usize = 16; // dht::LOOKUP_SIZE

///
#[derive(Copy, Clone, Debug)]
pub struct SearchInfo {
    dest: NodeID,
    mask: NodeID,
    time: Instant,
    // visited:
    /// The number of requests sent.
    sent: u64,
    /// The number of responses received.
    recv: u64,
}

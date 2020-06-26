use std::time::Duration;

///
const ROOT_TIMEOUT: Duration = Duration::from_secs(60);

///
const UPDATE_INTERVAL: Duration = Duration::from_secs(ROOT_TIMEOUT.as_secs() >> 1);

///
const THROTTLE_INTERVAL: Duration = Duration::from_secs(UPDATE_INTERVAL.as_secs() >> 1);

/// Number of switch updates before switching to a faster parent.
const PARENT_UPDATE_THRESHOLD: u8 = 240;

///
const MIN_TOTAL_QUEUE_SIZE: u64 = 4 * 1024 * 1024;

///
#[derive(Debug)]
pub struct Switch;

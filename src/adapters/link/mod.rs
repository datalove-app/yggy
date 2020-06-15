mod udp;

use std::time::Duration;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6);

lazy_static! {
    ///
    pub static ref PING_INTERVAL: Duration = (DEFAULT_TIMEOUT * 2) / 3;
}

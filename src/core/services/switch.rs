use crate::core::types::{SigningPublicKey, SwitchLocator, SwitchPort};

///
///
pub trait Switch {}

///
#[derive(Debug)]
pub struct SwitchData {
    locator: SwitchLocator,
    seq: u64,
    // peers:
    // msg: SwitchMessage
}

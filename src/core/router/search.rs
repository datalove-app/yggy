use super::Router;
use crate::{
    core_interfaces::router::search,
    core_types::{NodeID, SearchInfo},
    error::Error,
};

///
#[derive(Debug)]
pub struct SearchManager;

impl search::SearchManager for SearchManager {
    fn reconfigure(&mut self) {
        unimplemented!()
    }

    fn new_search(&self, dest: NodeID, mask: NodeID) -> Result<&SearchInfo, Error> {
        unimplemented!()
    }
}

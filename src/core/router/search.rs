use super::Router;
use crate::{
    core_interfaces::{router::search, Core},
    core_types::{NodeID, SearchInfo},
    error::Error,
};
use std::{collections::HashMap, time::Duration};
use xactor::{Actor, Addr};

///
const RESTART_TIMEOUT: Duration = Duration::from_secs(3);

///
const STEP_TIMEOUT: Duration = Duration::from_secs(1);

// ///
// const MAX_RESULTS: usize =

///
#[derive(Debug)]
pub struct SearchManager<C: Core> {
    ///
    router: Addr<<C as Core>::Router>,

    ///
    active_searches: HashMap<NodeID, SearchInfo>,
}

impl<C: Core> search::SearchManager for SearchManager<C> {
    fn reconfigure(&mut self) {
        unimplemented!()
    }

    fn new_search(&self, dest: NodeID, mask: NodeID) -> Result<&SearchInfo, Error> {
        unimplemented!()
    }
}

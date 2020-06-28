use super::Router;
use crate::{
    core_interfaces::{router::search, Core},
    core_types::NodeID,
    error::Error,
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
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
    router: Addr<<C as Core>::Router>,

    /// Ongoing searches.
    searches: HashMap<NodeID, <Self as search::SearchManager<C>>::Search>,
}

impl<C: Core> search::SearchManager<C> for SearchManager<C> {
    type Search = SearchInfo<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }

    // fn new_search(&self, dest: NodeID, mask: NodeID) -> Result<&SearchInfo, Error> {
    //     unimplemented!()
    // }
}

///
#[derive(Clone, Debug)]
pub struct SearchInfo<C: Core> {
    search_manager: Arc<SearchManager<C>>,
    dest: NodeID,
    mask: NodeID,
    time: Instant,
    // visited:
    //     /// The number of requests sent.
    //     sent: u64,
    //     /// The number of responses received.
    //     recv: u64,
}

impl<C: Core> search::Search<C, SearchManager<C>> for SearchInfo<C> {}

//!

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use yggy_core::{
    dev::*,
    interfaces::router::{search, Router as _},
    types::NodeID,
};

///
const RESTART_TIMEOUT: Duration = Duration::from_secs(3);

///
const STEP_TIMEOUT: Duration = Duration::from_secs(1);

// ///
// const MAX_RESULTS: usize =

///
#[derive(Debug)]
pub struct SearchManager<C: Core> {
    router: Addr<C::Router>,

    /// Ongoing searches.
    searches: HashMap<NodeID, SearchInfo<C>>,
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

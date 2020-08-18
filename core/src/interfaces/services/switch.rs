//! Maintains a spanning tree of a subset of the network, allowing us to route
//! packets based on distance between nodes in the tree, potentially taking
//! shortcuts if found.

use crate::{
    dev::*,
    types::{NodeID, SigningPublicKey, SwitchLocator, SwitchPort},
};
use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};

///
///
#[async_trait::async_trait]
pub trait Switch<C: Core>
where
    Self: Actor,
    Self: Handler<messages::GetLookupTable<C, Self>>,
{
    ///
    type LookupTable: LookupTable;

    /// Retrieves a copy of the `Switch`'s [`LookupTable`].
    ///
    /// [`LookupTable`]: trait.LookupTable.html
    async fn get_lookup_table(addr: &mut Addr<Self>) -> Result<Self::LookupTable, Error> {
        addr.call(messages::GetLookupTable::<C, Self>::MSG)
            .await
            .map_err(|_| Error::Init(anyhow::anyhow!("unable to retrieve lookup table")))
    }
}

/// Marker trait for the `Switch`'s inner lookup table.
pub trait LookupTable: Sized
where
    Self: Clone + fmt::Debug + Send + Sync,
{
    type Item;
}

pub mod messages {
    use super::*;
    use crate::dev::*;
    use std::marker::PhantomData;

    /// Retrieves the underlying `LookupTable`.
    pub struct GetLookupTable<C: Core, S: Switch<C>> {
        core: PhantomData<C>,
        switch: PhantomData<S>,
    }

    impl<C: Core, S: Switch<C>> GetLookupTable<C, S> {
        pub const MSG: Self = Self {
            core: PhantomData,
            switch: PhantomData,
        };
    }

    impl<C: Core, S: Switch<C>> Message for GetLookupTable<C, S> {
        type Result = S::LookupTable;
    }
}

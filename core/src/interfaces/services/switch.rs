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
    type LookupTable: LookupTable + Clone;

    async fn get_lookup_table(mut addr: Addr<Self>) -> Option<Self::LookupTable> {
        addr.call(messages::GetLookupTable::<C, Self>::new())
            .await
            .ok()
    }
}

/// Marker trait for the `Switch`'s inner lookup table.
pub trait LookupTable: 'static + fmt::Debug + Send + Sync {}

pub mod messages {
    use super::*;
    use std::marker::PhantomData;

    // #[derive(Debug)]
    pub struct GetLookupTable<C: Core, S: Switch<C>> {
        core: PhantomData<C>,
        switch: PhantomData<S>,
    }

    impl<C: Core, S: Switch<C>> GetLookupTable<C, S> {
        #[inline]
        pub fn new() -> Self {
            Self {
                core: PhantomData,
                switch: PhantomData,
            }
        }
    }

    impl<C: Core, S: Switch<C>> xactor::Message for GetLookupTable<C, S> {
        type Result = S::LookupTable;
    }
}

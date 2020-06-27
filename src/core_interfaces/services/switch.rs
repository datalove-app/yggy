use crate::{
    core_interfaces::Core,
    core_types::{NodeID, SigningPublicKey, SwitchLocator, SwitchPort},
};
use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};
use xactor::{Actor, Addr, Handler};

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

    async fn get_lookup_table(addr: &mut Addr<Self>) -> Arc<Self::LookupTable> {
        addr.call(messages::GetLookupTable::<C, Self>::new())
            .await
            .unwrap()
    }
}

/// Marker trait for the `Switch`'s inner lookup table.
pub trait LookupTable: 'static + fmt::Debug + Send + Sync {}

pub mod messages {
    use super::*;

    pub struct GetLookupTable<C: Core, S: Switch<C>> {
        core: std::marker::PhantomData<C>,
        switch: std::marker::PhantomData<S>,
    }

    impl<C: Core, S: Switch<C>> GetLookupTable<C, S> {
        pub fn new() -> Self {
            Self {
                core: std::marker::PhantomData,
                switch: std::marker::PhantomData,
            }
        }
    }

    impl<C: Core, S: Switch<C>> xactor::Message for GetLookupTable<C, S> {
        type Result = Arc<<S as Switch<C>>::LookupTable>;
    }
}

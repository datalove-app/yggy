use crate::core_interfaces::{Core, Session as ISession, SessionManager as ISessionManager};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};

///
#[derive(Debug)]
pub struct SessionManager<C: Core> {
    core: Addr<C>,
}

impl<C: Core> ISessionManager<C> for SessionManager<C> {
    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for SessionManager<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Session<C: Core> {
    core: Addr<C>,
    session_manager: Addr<SessionManager<C>>,
}

impl<C: Core> ISession<C, SessionManager<C>> for Session<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Session<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

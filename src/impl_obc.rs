use std::sync::Arc;

use walle_core::action::Action;
use walle_core::event::Event;
use walle_core::prelude::async_trait;
use walle_core::prelude::WalleResult;
use walle_core::resp::Resp;
use walle_core::ActionHandler;
use walle_core::EventHandler;
use walle_core::OneBot;

use crate::event::Event as V11Event;
use crate::parse::{action, event, resp};

pub struct ImplOBC {
    pub(crate) event_tx: tokio::sync::broadcast::Sender<V11Event>,
}

#[async_trait]
impl EventHandler<Event, Action, Resp> for ImplOBC {
    type Config = walle_core::config::ImplConfig;
    async fn start<AH, EH>(
        &self,
        _ob: &Arc<OneBot<AH, EH>>,
        _config: Self::Config,
    ) -> WalleResult<Vec<tokio::task::JoinHandle<()>>>
    where
        AH: ActionHandler<Event, Action, Resp> + Send + Sync + 'static,
        EH: EventHandler<Event, Action, Resp> + Send + Sync + 'static,
    {
        todo!()
    }
    async fn call(&self, event: Event) -> WalleResult<()> {
        self.event_tx.send(event::to_11(event)).ok();
        Ok(())
    }
    async fn before_call_action(&self, action: Action) -> WalleResult<Action> {
        Ok(action::to_11(action))
    }
    async fn after_call_action(&self, resp: Resp) -> WalleResult<Resp> {
        Ok(resp::to_12(resp))
    }
    async fn shutdown(&self) {}
}

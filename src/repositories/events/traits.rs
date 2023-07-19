use axum::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

#[async_trait]
pub trait EventTrait {
    async fn subscribe(&self) -> UnboundedReceiver<String>;
    async fn notify(&self, msg: String) -> anyhow::Result<()>;
}
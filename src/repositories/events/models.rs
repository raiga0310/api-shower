use axum::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{mpsc::UnboundedSender, mpsc::unbounded_channel, Mutex};
use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicU64, atomic::Ordering};

use crate::repositories::events::traits::EventTrait;

pub struct Events {
    clients: Arc<Mutex<HashMap<u64, UnboundedSender<String>>>>,
    last_id: AtomicU64
}

impl Events {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            last_id: AtomicU64::new(0)
        }
    }
}

#[async_trait]
impl EventTrait for Events {

    async fn subscribe(&self) -> UnboundedReceiver<String> {
        let (tx, rx) = unbounded_channel();
        let id = self.last_id.fetch_add(1, Ordering::SeqCst);
        self.clients.lock().await.insert(id, tx);
        rx
    }

    async fn notify(&self, msg: String) -> anyhow::Result<()> {
        let mut clients = self.clients.lock().await;
        clients.retain(|_, sender| sender.send(msg.clone()).is_ok());
        Ok(())
    }
}

#[cfg(test)]
mod events_test {
    use super::*;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_new() {
        let events = Events::new();
        assert_eq!(events.clients.lock().await.len(), 0);
        assert_eq!(events.last_id.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_subscribe() {
        let events = Events::new();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            events.subscribe().await;
            assert_eq!(events.clients.lock().await.len(), 1);
            assert_eq!(events.last_id.load(Ordering::SeqCst), 1);
        });
    }

    #[test]
    fn test_notify() {
        let events = Events::new();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut rx = events.subscribe().await;
            events.notify("test".to_string()).await.unwrap();
            assert_eq!(rx.recv().await.unwrap(), "test");
        });
    }
}


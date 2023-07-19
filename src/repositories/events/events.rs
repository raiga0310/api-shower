use tokio::sync::{mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel}, Mutex};
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
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

    pub async fn subscribe(&self) -> UnboundedReceiver<String> {
        let (tx, rx) = unbounded_channel();
        let id = self.last_id.fetch_add(1, Ordering::SeqCst);
        self.clients.lock().await.insert(id, tx);
        rx
    }

    pub async fn notify(&self, msg: String) {
        let mut clients = self.clients.lock().await;
        clients.retain(|_, sender| sender.send(msg.clone()).is_ok());
    }
}
use std::sync::Arc;
use axum::response::IntoResponse;
use hyper::Body;
use http::{Response, StatusCode};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

use crate::repositories::events::traits::EventTrait;

pub async fn server_sents_events(events: Arc<impl EventTrait>) -> Result<impl IntoResponse, StatusCode> {
    let rx = events.subscribe().await;
    let stream = UnboundedReceiverStream::new(rx).map(|msg| Ok::<_, hyper::Error>(format!("data: {}\n\n", msg)));

    let response = Response::builder()
        .header("Content-Type", "text/event-stream")
        .body(Body::wrap_stream(stream))
        .unwrap();
    Ok((StatusCode::OK, response))
}

#[cfg(test)]
mod test_sse_handler {
    use super::*;
    use std::sync::Arc;
    use axum::async_trait;
    use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

    use crate::repositories::events::traits::EventTrait;

    struct MockEvents {
        message: String,
    }

    impl MockEvents {
        pub async fn subscrive(&self) -> UnboundedReceiver<String> {
            let (tx, rx) = unbounded_channel();
            let _ = tx.send(self.message.clone());
            rx
        }
    }

    #[async_trait]
    impl EventTrait for MockEvents {
        async fn subscribe(&self) -> UnboundedReceiver<String> {
            self.subscrive().await
        }

        async fn notify(&self, _msg: String) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_server_sents_events() {
        let mock_events = Arc::new(MockEvents { message: "Hello!".to_string()});
        let result = server_sents_events(mock_events).await;
        let mut response = result.unwrap().into_response();

        assert_eq!(response.headers().get("Content-Type").unwrap(), "text/event-stream");
        let bytes = hyper::body::to_bytes(response.body_mut()).await.unwrap();
        assert_eq!(bytes, "data: Hello!\n\n".as_bytes());
    }
}

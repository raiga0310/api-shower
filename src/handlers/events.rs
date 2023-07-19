use std::sync::Arc;
use axum::response::IntoResponse;
use hyper::Body;
use http::{Response, StatusCode};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

use crate::repositories::events::events::Events;

pub async fn server_sents_events(events: Arc<Events>) -> Result<impl IntoResponse, StatusCode> {
    let rx = events.subscribe().await;
    let stream = UnboundedReceiverStream::new(rx).map(|msg| Ok::<_, hyper::Error>(format!("data: {}\n\n", msg)));

    let response = Response::builder()
        .header("Content-Type", "text/event-stream")
        .body(Body::wrap_stream(stream))
        .unwrap();
    Ok((StatusCode::OK, response))
}

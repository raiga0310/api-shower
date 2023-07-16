mod handlers;
mod repositories;

use crate::repositories::section::{InMemorySectionRepository, SectionRepository};
use handlers::section::{
    create_section, handler_404, root, showerrooms, showerrooms_building, showerrooms_gender,
};

use axum::{
    routing::{get, post},
    Router,
};
use std::{env, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    tracing_subscriber::fmt::init();

    let repository = InMemorySectionRepository::new();
    let app = create_app(repository);
    // add 404 handler
    let app = app.fallback(handler_404);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("mode: {}", log_level);
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<R: SectionRepository>(repository: R) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/:gender/showerrooms", get(showerrooms_gender))
        .route("/:gender/:building/showerrooms", get(showerrooms_building))
        .route(
            "/:gender/:building/:floor/showerrooms",
            get(showerrooms).post(create_section::<R>),
        )
        .with_state(Arc::new(repository))
}

#[cfg(test)]
mod tests {
    use crate::repositories::section::Section;

    use super::*;
    use axum::body::Body;
    use axum::http::Method;
    use axum::http::Request;
    use hyper::header;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_root() {
        let repository = InMemorySectionRepository::new();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "Hello, World!");
        // add more assertions if needed
    }

    // post section test case
    #[tokio::test]
    async fn should_return_section_data() {
        let repository = InMemorySectionRepository::new();
        let app = create_app(repository);
        let request_body = Body::from(r#"{"total": 10}"#);
        let request = Request::builder()
            .method(Method::POST)
            .uri("/female/C/1/showerrooms")
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(request_body)
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let section: Section =
            serde_json::from_str(&body).expect("Cannot convert Section instance.");
        assert_eq!(section.id, 1);
    }

    // add more tests for other routes
}

mod handlers;
mod repositories;

use crate::repositories::section::{InMemorySectionRepository, SectionRepository};
use handlers::section::{
    create_section, handler_404, root, showerrooms_building, showerrooms_floor, showerrooms_gender,
    update_section,
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
        .route("/:gender/showerrooms", get(showerrooms_gender::<R>))
        .route(
            "/:gender/:building/showerrooms",
            get(showerrooms_building::<R>),
        )
        .route(
            "/:gender/:building/:floor/showerrooms",
            get(showerrooms_floor::<R>)
                .post(create_section::<R>)
                .patch(update_section::<R>),
        )
        .with_state(Arc::new(repository))
}

#[cfg(test)]
mod tests {
    use crate::repositories::section::{CreateSection, Section, SectionInfo};

    use super::*;
    use axum::body::Body;
    use axum::http::Method;
    use axum::http::Request;
    use axum::http::StatusCode;
    use hyper::header;
    use tower::ServiceExt;

    // utility function to create populated repository
    fn create_populated_repository() -> InMemorySectionRepository {
        let mut repository = InMemorySectionRepository::new();
        let genders = vec!["male", "female"];
        let buildings = vec!["A", "B", "C"];
        let floors = vec![1, 2, 3, 4];

        for gender in genders {
            for building in &buildings {
                for &floor in &floors {
                    let section_info = SectionInfo {
                        gender: gender.to_string(),
                        building: building.to_string(),
                        floor,
                    };
                    let create_section = CreateSection { total: 5 };
                    repository.create(create_section, section_info);
                }
            }
        }

        repository
    }

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

    #[tokio::test]
    async fn test_showerrooms_gender() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // More detailed check on the response
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Section> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body.len(), 12); // 3 buildings * 4 floors
    }

    #[tokio::test]
    async fn test_showerrooms_building() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/C/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // More detailed check on the response
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Section> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body.len(), 4); // 1 building * 4 floors
    }

    #[tokio::test]
    async fn test_showerrooms_floor() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/C/1/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // More detailed check on the response
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Section = serde_json::from_slice(&bytes).unwrap();

        // Check if the returned section matches the expected gender, building, and floor
        assert_eq!(body.gender, "female");
        assert_eq!(body.building, "C");
        assert_eq!(body.floor, 1);
    }

    #[tokio::test]
    async fn test_invalid_gender() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/invalidgender/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invalid_building() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/invalidbuilding/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invalid_floor() {
        let repository = create_populated_repository();
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/C/5/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

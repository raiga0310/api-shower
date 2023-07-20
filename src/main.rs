mod handlers;
mod repositories;

use crate::repositories::{section::{db::DBSectionRepository, traits::SectionRepository}, events::models::Events};

use handlers::{section::{
    create_section, handler_404, root, showerrooms_all, showerrooms_building, showerrooms_floor,
    showerrooms_gender, update_section,
}, events::server_sents_events};

use axum::{routing::get, Router};
use dotenv::dotenv;
use hyper::{header, http::HeaderValue};
use sqlx::PgPool;
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

static EVENTS: once_cell::sync::Lazy<Arc<Events>> = once_cell::sync::Lazy::new(|| Arc::new(Events::new()));

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    tracing::info!("Starting server at: {}", database_url);
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("Failed to connect to {}", database_url));
    let repository = DBSectionRepository::new(pool.clone());

    let app = create_app(repository);
    // add 404 handler
    let app = app.fallback(handler_404);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("mode: {}", log_level);
    tracing::debug!("Listening on {}", addr);

    let graceful = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
        });

    if let Err(e) = graceful.await {
        tracing::error!("server error: {}", e);
    }

    let _ = tx.send(());
}

fn create_app<R: SectionRepository>(repository: R) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/showerrooms", get(showerrooms_all::<R>))
        .route("/:gender/showerrooms", get(showerrooms_gender::<R>))
        .route(
            "/:gender/:building/showerrooms",
            get(showerrooms_building::<R>),
        )
        .route(
            "/events", get({
                let events = Arc::clone(&EVENTS);
                move || server_sents_events(Arc::clone(&events))
            })
        )
        .route(
            "/:gender/:building/:floor/showerrooms",
            get(showerrooms_floor::<R>)
                .post(create_section::<R>)
                .patch(update_section::<R>),
        )
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT]),
        )
}

#[cfg(test)]
mod unite_tests {
    use crate::repositories::section::in_memory::InMemorySectionRepository;
    use crate::repositories::section::models::{CreateSection, Section, SectionInfo};

    use super::*;
    use axum::body::Body;
    use axum::http::Method;
    use axum::http::Request;
    use axum::http::StatusCode;
    use hyper::header;
    use tower::ServiceExt;

    // utility function to create populated repository
    async fn create_populated_repository() -> InMemorySectionRepository {
        let repository = InMemorySectionRepository {
            store: Arc::default(),
        };
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
                    repository
                        .create(create_section, section_info)
                        .await
                        .unwrap();
                }
            }
        }

        repository
    }

    #[tokio::test]
    async fn test_root() {
        let repository = InMemorySectionRepository {
            store: Arc::default(),
        };
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
        let repository = InMemorySectionRepository {
            store: Arc::default(),
        };
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
        let repository = create_populated_repository().await;
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
        let repository = create_populated_repository().await;
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
        let repository = create_populated_repository().await;
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
        let result: Vec<Section> = serde_json::from_slice(&bytes).unwrap();
        let body = &result[0];

        // Check if the returned section matches the expected gender, building, and floor
        assert_eq!(body.gender, "female");
        assert_eq!(body.building, "C");
        assert_eq!(body.floor, 1);
        assert_eq!(body.total, 5);
        assert_eq!(body.available, 5);
    }

    #[tokio::test]
    async fn test_invalid_gender() {
        let repository = create_populated_repository().await;
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
        let repository = create_populated_repository().await;
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
        let repository = create_populated_repository().await;
        let app = create_app(repository);
        let request = Request::builder()
            .method(Method::GET)
            .uri("/female/C/5/showerrooms")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_section() {
        let repository = create_populated_repository().await;
        let app = create_app(repository);
        let request_body = Body::from(r#""occupied""#);
        let request = Request::builder()
            .method(Method::PATCH)
            .uri("/male/A/1/showerrooms")
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(request_body)
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // More detailed check on the response
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Section = serde_json::from_slice(&bytes).unwrap();

        // Check if the returned section matches the expected gender, building, and floor
        assert_eq!(body.gender, "male");
        assert_eq!(body.building, "A");
        assert_eq!(body.floor, 1);
        assert_eq!(body.total, 5);
        assert_eq!(body.available, 4);
        assert_eq!(body.occupied, 1);
    }
}

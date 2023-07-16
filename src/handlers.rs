pub mod section;
use thiserror::Error;
#[derive(Error, Debug)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    UnexpectedError(String),
    #[error("Not Found: [{0}]")]
    NotFound(String),
    #[error("Already Exists: [{0}]")]
    AlreadyExists(String),
}

// Compare this snippet from src\main.rs:
// use axum::{
//     extract::Extension,
//     handler::{get, post},
//     Router,
// };
// use std::sync::Arc;
// use std::sync::RwLock;
//
// use crate::repositories::section::{InMemorySectionRepository, SectionRepository};
// use crate::repositories::RepositoryError;
// use crate::repositories::section::Section;
// use crate::repositories::section::CreateSection;
// use crate::repositories::section::SectionInfo;
// use crate::repositories::section::UpdateSection;
// use crate::repositories::section::Usage;
// use crate::handlers::section::handler_404;
// use crate::handlers::section::root;

// Compare this snippet from src\main.rs:
// #[tokio::main]
// async fn main() {
//     let repository = Arc::new(RwLock::new(InMemorySectionRepository::new()));
//     let app = create_app(repository);
//     // add 404 handler
//     let app = app.fallback(handler_404);
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     tracing::info!("mode: {}", log_level);
//     tracing::debug!("Listening on {}", addr);
//
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
//
// fn create_app<R: Repository>(repository: Arc<R>) -> Router {
//     Router::new()
//         .route("/hoge/:id", get(hoge::<R>))
//         .with_state(repository)
// }
// async fn hoge<R: Repository>(
//     Extension(repository): Extension<Arc<R>>,
//     path: extract::Path<(String,)>,
// ) -> Result<Json<Section>, Infallible> {
//     let (id,) = path.0;
//     let repository = repository.read().unwrap();
//     let section = repository.find_by_id(id).await.unwrap();
//     Ok(Json(section))
// }
//


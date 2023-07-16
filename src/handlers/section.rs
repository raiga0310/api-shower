use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::repositories::section::{SectionInfo, CreateSection, SectionRepository};


pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to here")
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn showerrooms_gender(Path(gender): Path<String>) -> String {
    format! {"Gender: {}", gender}
}

pub async fn showerrooms_building(Path((gender, building)): Path<(String, String)>) -> String {
    format! {"Gender: {}, Building: {:?}", gender, building}
}

pub async fn showerrooms(Path((gender, building, floor)): Path<(String, String, i32)>) -> String {
    format!(
        "Gender: {}, Building: {:?}, Floor: {:?}",
        gender, building, floor
    )
}

pub async fn create_section<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateSection>,
) -> impl IntoResponse {
    let info = SectionInfo {
        gender,
        building,
        floor,
    };
    let section = repository.create(payload, info);

    (StatusCode::CREATED, Json(section))
}
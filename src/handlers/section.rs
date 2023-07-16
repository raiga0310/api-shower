use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::repositories::section::{
    CreateSection, InMemorySectionRepository, SectionInfo, SectionRepository, UpdateSection,
};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to here")
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn showerrooms_gender<R: SectionRepository>(
    Path(gender): Path<String>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    //use find_by gender
    let sections = repository
        .find_by_gender(gender)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn showerrooms_building<R: SectionRepository>(
    Path((gender, building)): Path<(String, String)>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let sections = repository
        .find_by_building(gender, building)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn showerrooms_floor<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let sections = repository
        .find_by_floor(gender, building, floor)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn create_section<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateSection>,
) -> Result<impl IntoResponse, StatusCode> {
    let info = SectionInfo {
        gender,
        building,
        floor,
    };
    let section = repository.create(payload, info);

    Ok((StatusCode::CREATED, Json(section)))
}

pub async fn update_section<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
    Json(payload): Json<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // first get the id of the section
    let id = repository
        .find_by_floor(gender, building, floor)
        .unwrap()
        .id;
    let section = UpdateSection {
        id,
        status: payload,
    };
    let section = repository.update(section).unwrap();

    Ok((StatusCode::OK, Json(section)))
}

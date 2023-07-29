use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    repositories::{
        events::traits::EventTrait,
        section::{
            models::{CreateSection, SectionInfo, UpdatePayload, UpdateSection},
            traits::SectionRepository,
        },
    },
    EVENTS,
};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to here")
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn showerrooms_all<R: SectionRepository>(
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let sections = repository.find_all().await.unwrap();
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn showerrooms_gender<R: SectionRepository>(
    Path(gender): Path<String>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    //use find_by gender
    let sections = repository
        .find_by_gender(gender)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn showerrooms_building<R: SectionRepository>(
    Path((gender, building)): Path<(String, String)>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let sections = repository
        .find_by_building(gender, building)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(sections)))
}

pub async fn showerrooms_floor<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let sections = repository
        .find_by_floor(gender, building, floor)
        .await
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
    let section = repository.create(payload, info).await.unwrap();

    Ok((StatusCode::CREATED, Json(section)))
}

pub async fn update_section<R: SectionRepository>(
    Path((gender, building, floor)): Path<(String, String, i32)>,
    State(repository): State<Arc<R>>,
    Json(payload): Json<UpdatePayload>,
) -> Result<impl IntoResponse, StatusCode> {
    // first get the id of the section
    let id = repository
        .find_by_floor(gender.clone(), building.clone(), floor)
        .await
        .unwrap()
        .first()
        .unwrap()
        .id;
    let section = UpdateSection {
        id,
        current_status: payload.current_status,
        next_status: payload.next_status,
    };
    let section = repository.update(section).await.unwrap();

    // if section update is successful, notify the event
    let events = Arc::clone(&EVENTS);
    let msg = format!("{}/{}/{}", gender, building, floor);
    events.notify(msg).await.unwrap();

    Ok((StatusCode::OK, Json(section)))
}

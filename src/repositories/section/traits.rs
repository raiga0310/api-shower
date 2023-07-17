use crate::repositories::section::models::{CreateSection, Section, SectionInfo, UpdateSection};
use axum::async_trait;

#[async_trait]
pub trait SectionRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Section>;
    async fn find_by_gender(&self, gender: String) -> anyhow::Result<Vec<Section>>;
    async fn find_by_building(
        &self,
        gender: String,
        building: String,
    ) -> anyhow::Result<Vec<Section>>;
    async fn find_by_floor(
        &self,
        gender: String,
        building: String,
        floor: i32,
    ) -> anyhow::Result<Vec<Section>>;
    async fn find_all(&self) -> anyhow::Result<Vec<Section>>;
    async fn create(&self, section: CreateSection, info: SectionInfo) -> anyhow::Result<Section>;
    async fn update(&self, section: UpdateSection) -> anyhow::Result<Section>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

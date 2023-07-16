use crate::repositories::section::models::{CreateSection, Section, SectionInfo, UpdateSection};
use crate::repositories::section::traits::SectionRepository;
use axum::async_trait;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct DBSectionRepository {
    pool: PgPool,
}

impl DBSectionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SectionRepository for DBSectionRepository {
    // use todo!()
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Section> {
        let section = sqlx::query_as::<_, Section>(
            "SELECT * FROM sections WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(section)
    }

    async fn find_by_gender(&self, gender: String) -> anyhow::Result<Vec<Section>> {
        todo!()
    }

    async fn find_by_building(
        &self,
        gender: String,
        building: String,
    ) -> anyhow::Result<Vec<Section>> {
        todo!()
    }

    async fn find_by_floor(
        &self,
        gender: String,
        building: String,
        floor: i32,
    ) -> anyhow::Result<Section> {
        todo!()
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Section>> {
        todo!()
    }

    async fn create(&self, section: CreateSection, info: SectionInfo) -> anyhow::Result<Section> {
        todo!()
    }

    async fn update(&self, section: UpdateSection) -> anyhow::Result<Section> {
        todo!()
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}

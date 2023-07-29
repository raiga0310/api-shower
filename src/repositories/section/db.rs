use crate::repositories::section::models::{CreateSection, Section, SectionInfo, UpdateSection};
use crate::repositories::section::traits::SectionRepository;
use axum::async_trait;
use sqlx::PgPool;

use super::utils::query_switch_usage;

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
        let section = sqlx::query_as::<_, Section>("SELECT * FROM sections WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(section)
    }

    async fn find_by_gender(&self, gender: String) -> anyhow::Result<Vec<Section>> {
        let sections = sqlx::query_as::<_, Section>(
            "SELECT * FROM sections WHERE gender = $1 order by id asc",
        )
        .bind(gender)
        .fetch_all(&self.pool)
        .await?;
        Ok(sections)
    }

    async fn find_by_building(
        &self,
        gender: String,
        building: String,
    ) -> anyhow::Result<Vec<Section>> {
        let sections = sqlx::query_as::<_, Section>(
            "SELECT * FROM sections WHERE gender = $1 AND building = $2 order by id asc",
        )
        .bind(gender)
        .bind(building)
        .fetch_all(&self.pool)
        .await?;
        Ok(sections)
    }

    async fn find_by_floor(
        &self,
        gender: String,
        building: String,
        floor: i32,
    ) -> anyhow::Result<Vec<Section>> {
        let sections = sqlx::query_as::<_, Section>(
            "SELECT * FROM sections WHERE gender = $1 AND building = $2 AND floor = $3",
        )
        .bind(gender)
        .bind(building)
        .bind(floor)
        .fetch_one(&self.pool)
        .await?;
        Ok(vec![sections])
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Section>> {
        let sections = sqlx::query_as::<_, Section>("SELECT * FROM sections order by id asc")
            .fetch_all(&self.pool)
            .await?;
        Ok(sections)
    }

    async fn create(&self, section: CreateSection, info: SectionInfo) -> anyhow::Result<Section> {
        let section = sqlx::query_as::<_, Section>(
            "insert into sections (building, floor, gender, total, available, occupied, disabled_rooms) values ($1, $2, $3, $4, $4, 0, 0) returning *)"
        )
        .bind(info.building)
        .bind(info.floor)
        .bind(info.gender)
        .bind(section.total)
        .fetch_one(&self.pool)
        .await?;

        let section = self.find_by_id(section.id).await?;
        Ok(section)
    }

    async fn update(&self, section: UpdateSection) -> anyhow::Result<Section> {
        let query = query_switch_usage(section.current_status, section.next_status).unwrap();
        let section = sqlx::query_as::<_, Section>(query)
            .bind(section.id)
            .fetch_one(&self.pool)
            .await?;

        let section = self.find_by_id(section.id).await?;
        Ok(section)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let _ = sqlx::query_as::<_, Section>("delete from sections where id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::section::models::UpdateSection;
    use crate::repositories::section::traits::SectionRepository;
    use anyhow::Result;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    async fn setup() -> Result<DBSectionRepository> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let pool = PgPool::connect(&database_url).await?;
        let repository = DBSectionRepository::new(pool);
        Ok(repository)
    }

    #[tokio::test]
    async fn test_find_by_id() -> Result<()> {
        let repository = setup().await?;

        let section = repository.find_by_id(1).await?;
        // Assert based on your known test data
        assert_eq!(section.id, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_gender() -> Result<()> {
        let repository = setup().await?;

        let sections = repository.find_by_gender("male".to_string()).await?;
        // Assert based on your known test data
        assert_eq!(sections[0].gender, "male");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_building() -> Result<()> {
        let repository = setup().await?;

        let sections = repository
            .find_by_building("male".to_string(), "A".to_string())
            .await?;
        // Assert based on your known test data
        assert_eq!(sections[0].building, "A");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_floor() -> Result<()> {
        let repository = setup().await?;

        let sections = repository
            .find_by_floor("male".to_string(), "A".to_string(), 1)
            .await?;
        // Assert based on your known test data
        assert_eq!(sections[0].floor, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_available() -> Result<()> {
        let repository = setup().await?;

        let update_section = UpdateSection {
            id: 1, // Some example id
            current_status: "available".to_string(),
            next_status: "occupied".to_string(),
        };
        let old_section = repository.find_by_id(update_section.id).await?;
        let updated_section = repository.update(update_section).await?;
        // Assert based on your known test data
        assert_eq!(updated_section.occupied, old_section.occupied + 1);
        assert_eq!(updated_section.available, old_section.available - 1);

        let update_section = UpdateSection {
            id: 1,
            current_status: "occupied".to_string(),
            next_status: "available".to_string(),
        };
        let old_section = repository.find_by_id(update_section.id).await?;
        let updated_section = repository.update(update_section).await?;
        // Assert based on your known test data
        assert_eq!(updated_section.occupied, old_section.occupied - 1);
        assert_eq!(updated_section.available, old_section.available + 1);

        let update_section = UpdateSection {
            id: 1, // Some example id
            current_status: "available".to_string(),
            next_status: "disabled".to_string(), // Some example status
        };
        let old_section = repository.find_by_id(update_section.id).await?;
        let updated_section = repository.update(update_section).await?;
        // Assert based on your known test data
        assert_eq!(
            updated_section.disabled_rooms,
            old_section.disabled_rooms + 1
        );
        assert_eq!(updated_section.available, old_section.available - 1);

        Ok(())
    }
}

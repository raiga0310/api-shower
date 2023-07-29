use crate::repositories::section::errors::RepositoryError;
use crate::repositories::section::models::{CreateSection, Section, SectionInfo, UpdateSection};
use crate::repositories::section::traits::SectionRepository;
use crate::repositories::section::utils::inmemory_switch_usage;
use anyhow::Context;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

type SecctionDatas = HashMap<i32, Section>;

#[derive(Clone, Debug)]
pub struct InMemorySectionRepository {
    pub store: Arc<RwLock<SecctionDatas>>,
}

impl InMemorySectionRepository {
    pub fn write_store_ref(&self) -> RwLockWriteGuard<SecctionDatas> {
        self.store.write().unwrap()
    }

    pub fn read_store_ref(&self) -> RwLockReadGuard<SecctionDatas> {
        self.store.read().unwrap()
    }
}

#[async_trait]
impl SectionRepository for InMemorySectionRepository {
    // using todo!() instead of implementing
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Section> {
        let store = self.read_store_ref();
        let section = store.get(&id).unwrap().clone();
        Ok(section)
    }
    async fn find_by_gender(&self, gender: String) -> anyhow::Result<Vec<Section>> {
        let store = self.read_store_ref();
        let sections = Vec::from_iter(
            store
                .values()
                .filter(|section| section.gender == gender)
                .cloned(),
        );

        if sections.is_empty() {
            Err(anyhow::Error::msg("No section found for the given gender"))
        } else {
            Ok(sections)
        }
    }
    async fn find_by_building(
        &self,
        gender: String,
        building: String,
    ) -> anyhow::Result<Vec<Section>> {
        let store = self.read_store_ref();
        let sections = Vec::from_iter(
            store
                .values()
                .filter(|section| section.gender == gender && section.building == building)
                .cloned(),
        );

        if sections.is_empty() {
            Err(anyhow::Error::msg(
                "No section found for the given gender or building",
            ))
        } else {
            Ok(sections)
        }
    }
    async fn find_by_floor(
        &self,
        gender: String,
        building: String,
        floor: i32,
    ) -> anyhow::Result<Vec<Section>> {
        let store = self.read_store_ref();
        let sections = Vec::from_iter(
            store
                .values()
                .filter(|section| {
                    section.gender == gender
                        && section.building == building
                        && section.floor == floor
                })
                .cloned(),
        );

        if sections.is_empty() {
            Err(anyhow::Error::msg(
                "No section found for the given gender or building",
            ))
        } else {
            Ok(sections.clone())
        }
    }
    async fn find_all(&self) -> anyhow::Result<Vec<Section>> {
        let store = self.read_store_ref();
        Ok(Vec::from_iter(store.values().map(|sec| sec.clone())))
    }
    async fn create(&self, payload: CreateSection, info: SectionInfo) -> anyhow::Result<Section> {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let section = Section::new(id, info.gender, info.building, info.floor, payload.total);
        store.insert(id, section.clone());
        Ok(section)
    }
    async fn update(&self, payload: UpdateSection) -> anyhow::Result<Section> {
        let mut store = self.write_store_ref();
        let section = store
            .get(&payload.id)
            .context(RepositoryError::NotFound(payload.id))?;
        let usage =
            inmemory_switch_usage(payload.current_status, payload.next_status, section.clone())?;
        let section = Section {
            id: payload.id,
            gender: section.gender.clone(),
            building: section.building.clone(),
            floor: section.floor,
            total: section.total,
            available: usage.available,
            occupied: usage.occupied,
            disabled_rooms: usage.disabled_rooms,
        };
        store.insert(payload.id, section.clone());
        Ok(section)
    }
    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod in_memory_tests {
    use super::*;

    #[tokio::test]
    async fn test_section_repository() {
        let repo = InMemorySectionRepository {
            store: Arc::default(),
        };

        // 1. Sectionの作成
        let create_section = CreateSection { total: 10 };
        let section_info = SectionInfo {
            gender: "male".to_string(),
            building: "A".to_string(),
            floor: 1,
        };
        let created_section = repo
            .create(create_section, section_info)
            .await
            .expect("failed to create section");
        assert_eq!(created_section.id, 1);
        assert_eq!(created_section.total, 10);

        // 2. 1で作成したSectionをidで取得
        let section = repo.find_by_id(1).await.unwrap();
        assert_eq!(section.id, 1);
        assert_eq!(section.total, 10);

        // 3. すべてのSectionを取得
        let sections = repo.find_all().await.unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].id, 1);
        assert_eq!(sections[0].total, 10);

        // 4. updateで更新(status更新は6パターン(うち2パターン))
        // 4.1. status = "occupied"
        let update_section = UpdateSection {
            id: 1,
            current_status: "available".to_string(),
            next_status: "occupied".to_string(),
        };
        let updated_section = repo.update(update_section).await.unwrap();
        assert_eq!(updated_section.available, 9);
        assert_eq!(updated_section.occupied, 1);

        // 4.2. status = "available"
        let update_section = UpdateSection {
            id: 1,
            current_status: "occupied".to_string(),
            next_status: "available".to_string(),
        };
        let updated_section = repo.update(update_section).await.unwrap();
        assert_eq!(updated_section.available, 10);
        assert_eq!(updated_section.occupied, 0);

        // 5. deleteで1で作成したSectionを削除
        let res = repo.delete(1).await;
        assert!(res.is_ok());
    }
}

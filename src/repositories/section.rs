use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("not found id is: {0}")]
    NotFound(i32),
}

pub trait SectionRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn find_by_id(&self, id: i32) -> Option<Box<Section>>;
    fn find_all(&self) -> Vec<Section>;
    fn create(&self, section: CreateSection, info: SectionInfo) -> Section;
    fn update(&self, id: i32, section: UpdateSection) -> anyhow::Result<Section>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: i32,
    gender: String,
    building: String,
    floor: i32,
    total: i32,
    available: i32,
    occupied: i32,
    disabled: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SectionInfo {
    pub gender: String,
    pub building: String,
    pub floor: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateSection {
    total: i32,
}

pub struct UpdateSection {
    id: i32,
    status: String,
}

pub struct Usage {
    available: i32,
    occupied: i32,
    disabled: i32,
}

impl Section {
    fn new(id: i32, gender: String, building: String, floor: i32, total: i32) -> Self {
        Self {
            id,
            gender,
            building,
            floor,
            total,
            available: total,
            occupied: 0,
            disabled: 0,
        }
    }
}

type SecctionDatas = HashMap<i32, Section>;

#[derive(Clone, Debug)]
pub struct InMemorySectionRepository {
    store: Arc<RwLock<SecctionDatas>>,
}

impl InMemorySectionRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }

    pub fn write_store_ref(&self) -> RwLockWriteGuard<SecctionDatas> {
        self.store.write().unwrap()
    }

    pub fn read_store_ref(&self) -> RwLockReadGuard<SecctionDatas> {
        self.store.read().unwrap()
    }
}

impl SectionRepository for InMemorySectionRepository {
    // using todo!() instead of implementing
    fn find_by_id(&self, id: i32) -> Option<Box<Section>> {
        let store = self.read_store_ref();
        let section = store.get(&id)?;
        let section = Box::new(section.clone());
        Some(section)
    }
    fn find_all(&self) -> Vec<Section> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|sec| sec.clone()))
    }
    fn create(&self, payload: CreateSection, info: SectionInfo) -> Section {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let section = Section::new(id, info.gender, info.building, info.floor, payload.total);
        store.insert(id, section.clone());
        section
    }
    fn update(&self, id: i32, payload: UpdateSection) -> anyhow::Result<Section> {
        let mut store = self.write_store_ref();
        let section = store
            .get(&payload.id)
            .context(RepositoryError::NotFound(payload.id))?;
        let usage = switch_usage(payload.status, section.clone())?;
        let section = Section {
            id,
            gender: section.gender.clone(),
            building: section.building.clone(),
            floor: section.floor,
            total: section.total,
            available: usage.available,
            occupied: usage.occupied,
            disabled: usage.disabled,
        };
        store.insert(payload.id, section.clone());
        Ok(section)
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

fn switch_usage(status: String, section: Section) -> anyhow::Result<Usage> {
    match status.as_str() {
        "available" => Ok(Usage {
            available: section.available + 1,
            occupied: section.occupied - 1,
            disabled: section.disabled,
        }),
        "occupied" => Ok(Usage {
            available: section.available - 1,
            occupied: section.occupied + 1,
            disabled: section.disabled,
        }),
        "disabled" => Ok(Usage {
            available: section.available - 1,
            occupied: section.occupied,
            disabled: section.disabled + 1,
        }),
        _ => Err(anyhow::anyhow!("invalid status")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_repository() {
        let repo = InMemorySectionRepository::new();

        // 1. Sectionの作成
        let create_section = CreateSection { total: 10 };
        let section_info = SectionInfo {
            gender: "male".to_string(),
            building: "A".to_string(),
            floor: 1,
        };
        let created_section = repo.create(create_section, section_info);
        assert_eq!(created_section.id, 1);
        assert_eq!(created_section.total, 10);

        // 2. 1で作成したSectionをidで取得
        let section = repo.find_by_id(1).unwrap();
        assert_eq!(section.id, 1);
        assert_eq!(section.total, 10);

        // 3. すべてのSectionを取得
        let sections = repo.find_all();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].id, 1);
        assert_eq!(sections[0].total, 10);

        // 4. updateで更新(status は2パターン)
        // 4.1. status = "available"
        let update_section = UpdateSection {
            id: 1,
            status: "available".to_string(),
        };
        let updated_section = repo.update(1, update_section).unwrap();
        assert_eq!(updated_section.available, 11);
        assert_eq!(updated_section.occupied, -1);

        // 4.2. status = "occupied"
        let update_section = UpdateSection {
            id: 1,
            status: "occupied".to_string(),
        };
        let updated_section = repo.update(1, update_section).unwrap();
        assert_eq!(updated_section.available, 10);
        assert_eq!(updated_section.occupied, 0);

        // 5. deleteで1で作成したSectionを削除
        repo.delete(1).unwrap();
        let section = repo.find_by_id(1);
        assert!(section.is_none());
    }
}

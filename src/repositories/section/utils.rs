use crate::repositories::section::models::{Section, Usage};

pub fn inmemory_switch_usage(
    current_status: String,
    next_status: String,
    section: Section,
) -> anyhow::Result<Usage> {
    match current_status.as_str() {
        "available" => {
            if next_status == "occupied" {
                if section.available > 0 {
                    Ok(Usage {
                        available: section.available - 1,
                        occupied: section.occupied + 1,
                        disabled_rooms: section.disabled_rooms,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are available"))
                }
            } else if next_status == "disabled" {
                if section.available > 0 {
                    Ok(Usage {
                        available: section.available - 1,
                        occupied: section.occupied,
                        disabled_rooms: section.disabled_rooms + 1,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are available to disable"))
                }
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        "occupied" => {
            if next_status == "available" {
                if section.occupied > 0 {
                    Ok(Usage {
                        available: section.available + 1,
                        occupied: section.occupied - 1,
                        disabled_rooms: section.disabled_rooms,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are occupied"))
                }
            } else if next_status == "disabled" {
                if section.occupied > 0 {
                    Ok(Usage {
                        available: section.available,
                        occupied: section.occupied - 1,
                        disabled_rooms: section.disabled_rooms + 1,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are occupied to disable"))
                }
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        "disabled" => {
            if next_status == "available" {
                if section.disabled_rooms > 0 {
                    Ok(Usage {
                        available: section.available + 1,
                        occupied: section.occupied,
                        disabled_rooms: section.disabled_rooms - 1,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are disabled"))
                }
            } else if next_status == "occupied" {
                if section.disabled_rooms > 0 {
                    Ok(Usage {
                        available: section.available,
                        occupied: section.occupied + 1,
                        disabled_rooms: section.disabled_rooms - 1,
                    })
                } else {
                    Err(anyhow::anyhow!("No more sections are disabled to occupy"))
                }
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        _ => Err(anyhow::anyhow!("invalid status")),
    }
}

pub fn query_switch_usage(
    current_status: String,
    next_status: String,
) -> anyhow::Result<&'static str> {
    match current_status.as_str() {
        "available" => {
            if next_status == "occupied" {
                Ok("update sections set available = available - 1, occupied = occupied + 1 where id = $1 returning *")
            } else if next_status == "disabled" {
                Ok("update sections set available = available - 1, disabled_rooms = disabled_rooms + 1 where id = $1 returning *")
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        "occupied" => {
            if next_status == "available" {
                Ok("update sections set available = available + 1, occupied = occupied - 1 where id = $1 returning *")
            } else if next_status == "disabled" {
                Ok("update sections set occupied = occupied - 1, disabled_rooms = disabled_rooms + 1 where id = $1 returning *")
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        "disabled" => {
            if next_status == "available" {
                Ok("update sections set available = available + 1, disabled_rooms = disabled_rooms - 1 where id = $1 returning *")
            } else if next_status == "occupied" {
                Ok("update sections set occupied = occupied + 1, disabled_rooms = disabled_rooms - 1 where id = $1 returning *")
            } else {
                Err(anyhow::anyhow!("invalid status"))
            }
        }
        _ => Err(anyhow::anyhow!("invalid status")),
    }
}

#[cfg(test)]
mod utils_test {
    use super::*;
    //use crate::repositories::section::models::Usage;

    #[test]
    fn test_inmemory_switch_usage() {
        let section = Section {
            id: 1,
            available: 5,
            occupied: 4,
            disabled_rooms: 1,
            gender: "male".to_string(),
            building: "A".to_string(),
            floor: 4,
            total: 10,
        };
        //a -> o
        let usage = inmemory_switch_usage(
            "available".to_string(),
            "occupied".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 4);
        assert_eq!(usage.occupied, 5);
        assert_eq!(usage.disabled_rooms, 1);
        //o -> d
        let usage = inmemory_switch_usage(
            "occupied".to_string(),
            "disabled".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 5);
        assert_eq!(usage.occupied, 3);
        assert_eq!(usage.disabled_rooms, 2);
        //d -> o
        let usage = inmemory_switch_usage(
            "disabled".to_string(),
            "occupied".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 5);
        assert_eq!(usage.occupied, 5);
        assert_eq!(usage.disabled_rooms, 0);
        //o -> a
        let usage = inmemory_switch_usage(
            "occupied".to_string(),
            "available".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 6);
        assert_eq!(usage.occupied, 3);
        assert_eq!(usage.disabled_rooms, 1);
        //a -> d
        let usage = inmemory_switch_usage(
            "available".to_string(),
            "disabled".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 4);
        assert_eq!(usage.occupied, 4);
        assert_eq!(usage.disabled_rooms, 2);
        //d -> a
        let usage = inmemory_switch_usage(
            "disabled".to_string(),
            "available".to_string(),
            section.clone(),
        )
        .unwrap();
        assert_eq!(usage.available, 6);
        assert_eq!(usage.occupied, 4);
        assert_eq!(usage.disabled_rooms, 0);
    }

    #[test]
    fn test_query() {
        let query = query_switch_usage("available".to_string(), "occupied".to_string()).unwrap();
        assert_eq!(query, "update sections set available = available - 1, occupied = occupied + 1 where id = $1 returning *");

        let query = query_switch_usage("available".to_string(), "disabled".to_string()).unwrap();
        assert_eq!(query, "update sections set available = available - 1, disabled_rooms = disabled_rooms + 1 where id = $1 returning *");

        let query = query_switch_usage("occupied".to_string(), "available".to_string()).unwrap();
        assert_eq!(query, "update sections set available = available + 1, occupied = occupied - 1 where id = $1 returning *");

        let query = query_switch_usage("occupied".to_string(), "disabled".to_string()).unwrap();
        assert_eq!(query, "update sections set occupied = occupied - 1, disabled_rooms = disabled_rooms + 1 where id = $1 returning *");

        let query = query_switch_usage("disabled".to_string(), "available".to_string()).unwrap();
        assert_eq!(query, "update sections set available = available + 1, disabled_rooms = disabled_rooms - 1 where id = $1 returning *");

        let query = query_switch_usage("disabled".to_string(), "occupied".to_string()).unwrap();
        assert_eq!(query, "update sections set occupied = occupied + 1, disabled_rooms = disabled_rooms - 1 where id = $1 returning *");
    }
}

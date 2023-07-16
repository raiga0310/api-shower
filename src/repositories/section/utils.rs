use crate::repositories::section::models::{Section, Usage};

pub fn switch_usage(status: String, section: Section) -> anyhow::Result<Usage> {
    match status.as_str() {
        "available" => {
            if section.occupied > 0 {
                Ok(Usage {
                    available: section.available + 1,
                    occupied: section.occupied - 1,
                    disabled: section.disabled,
                })
            } else {
                Err(anyhow::anyhow!("No more sections are occupied"))
            }
        }
        "occupied" => {
            if section.available > 0 {
                Ok(Usage {
                    available: section.available - 1,
                    occupied: section.occupied + 1,
                    disabled: section.disabled,
                })
            } else {
                Err(anyhow::anyhow!("No more sections are available"))
            }
        }
        "disabled" => {
            if section.available > 0 {
                Ok(Usage {
                    available: section.available - 1,
                    occupied: section.occupied,
                    disabled: section.disabled + 1,
                })
            } else {
                Err(anyhow::anyhow!("No more sections are available to disable"))
            }
        }
        _ => Err(anyhow::anyhow!("invalid status")),
    }
}

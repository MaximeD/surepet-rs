#[cfg(not(test))]
use chrono::Local;
use chrono::{DateTime, FixedOffset};
#[cfg(not(test))]
use chrono::SecondsFormat;
use dyn_partial_eq::*;
use humantime::format_duration;

use super::shared::Information;

/// Represent a pet.
#[derive(Debug, Eq, DynPartialEq, PartialEq)]
pub struct Pet {
    /// Name of the pet.
    pub name: String,
    /// Whether the pet is inside (1) or outside (2).
    pub position: u8,
    /// Since when the pet is inside / outside.
    pub position_since: DateTime<FixedOffset>,
}

impl Information for Pet {
    /// Summary of pet.
    ///
    /// Example:
    /// ```
    /// 🏠 Garfield is inside since 1h 27m 8s
    /// ```
    fn information(&self) -> String {
        match self.position {
            1 => format!(
                "🏠 {} is inside since {}",
                self.name,
                position_duration(self.position_since)
            ),
            2 => format!(
                "🏡 {} is outside since {}",
                self.name,
                position_duration(self.position_since)
            ),
            _ => panic!("Invalid pet position"),
        }
    }
}

/// Human readable duration since the pet position has not changed.
///
/// Example:
/// ```
/// 2h 12m 53s
/// ```
fn position_duration(since: DateTime<FixedOffset>) -> String {
    #[cfg(not(test))]
    // Convert to rfc3339 to remove the milliseconds.
    let now = Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    #[cfg(test)]
    let now = "2023-01-01T12:00:00+00:00";

    return format_duration(
        now.parse::<DateTime<FixedOffset>>()
            .unwrap()
            .signed_duration_since(since)
            .to_std()
            .unwrap(),
    )
    .to_string();
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn it_formats_information_when_pet_is_inside() {
        assert_eq!(
            Pet {
                name: "Garfield".to_string(),
                position: 1,
                position_since: "2023-01-01T10:32:52+00:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap()
            }
            .information(),
            "🏠 Garfield is inside since 1h 27m 8s"
        );
    }

    #[test]
    fn it_formats_information_when_pet_is_outside() {
        assert_eq!(
            Pet {
                name: "Garfield".to_string(),
                position: 2,
                position_since: "2022-12-28T16:09:52+00:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap()
            }
            .information(),
            "🏡 Garfield is outside since 3days 19h 50m 8s"
        );
    }
}

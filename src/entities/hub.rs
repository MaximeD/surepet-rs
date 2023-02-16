use dyn_partial_eq::*;

use super::shared::Information;

/// The surepet hub, responsible of collecting information from other devices.
#[derive(Debug, DynPartialEq, PartialEq)]
pub struct Hub {
    pub name: String,
    pub online: bool,
}

impl Information for Hub {
    /// Summary of the hub.
    ///
    /// Example:
    /// ```
    /// ✅ Hub is online
    /// ```
    fn information(&self) -> String {
        if self.online {
            format!("✅ {} is online", self.name)
        } else {
            format!("❌ {} is disconnected", self.name)
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn it_formats_information_when_hub_is_online() {
        assert_eq!(
            Hub {
                name: "Hub".to_string(),
                online: true,
            }
            .information(),
            "✅ Hub is online"
        );
    }

    #[test]
    fn it_formats_information_when_hub_is_disconnected() {
        assert_eq!(
            Hub {
                name: "Hub".to_string(),
                online: false,
            }
            .information(),
            "❌ Hub is disconnected"
        );
    }
}

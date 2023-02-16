use dyn_partial_eq::*;

use super::shared::Information;

/// When battery is full.
const BATTERY_VOLTAGE_FULL: f64 = 1.6;
/// When battery is low.
const BATTERY_VOLTAGE_LOW: f64 = 1.2;
/// How many batteries there are in the flap.
const BATTERIES_COUNT: u8 = 4;

/// The device pets use to go in and out.
#[derive(Debug, DynPartialEq, PartialEq)]
pub struct Flap {
    pub name: String,
    pub online: bool,
    pub battery_voltage: f64,
}

/// Remaining battery percentage level.
trait BatteryPercent {
    fn battery_percent(&self) -> f64;
}

impl BatteryPercent for Flap {
    fn battery_percent(&self) -> f64 {
        let voltage_diff = BATTERY_VOLTAGE_FULL - BATTERY_VOLTAGE_LOW;
        let voltage_per_battery = self.battery_voltage / BATTERIES_COUNT as f64;
        let voltage_per_battery_diff = voltage_per_battery - BATTERY_VOLTAGE_LOW;

        return (voltage_per_battery_diff / voltage_diff * 100.0)
            .min(100.0)
            .max(0.0);
    }
}

impl Information for Flap {
    /// Summary of a flap.
    ///
    /// Example:
    /// ```
    /// ✅ Flap is online (battery: 51.09%)
    /// ```
    fn information(&self) -> String {
        if self.online {
            if self.battery_percent() > 10.0 {
                format!(
                    "✅ {} is online (battery: {:.2}%)",
                    self.name,
                    self.battery_percent()
                )
            } else {
                format!(
                    "🪫 {} is online (battery: {:.2}%)",
                    self.name,
                    self.battery_percent()
                )
            }
        } else {
            format!("❌ {} is disconnected", self.name)
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn it_formats_information_when_flap_is_online() {
        assert_eq!(
            Flap {
                name: "Flap".to_string(),
                online: true,
                battery_voltage: 5.6175
            }
            .information(),
            "✅ Flap is online (battery: 51.09%)"
        );
    }

    #[test]
    fn it_formats_information_when_flap_battery_is_low() {
        assert_eq!(
            Flap {
                name: "Flap".to_string(),
                online: true,
                battery_voltage: 4.9213
            }
            .information(),
            "🪫 Flap is online (battery: 7.58%)"
        );
    }

    #[test]
    fn it_formats_information_when_flap_is_disconnected() {
        assert_eq!(
            Flap {
                name: "Flap".to_string(),
                online: false,
                battery_voltage: 5.6175
            }
            .information(),
            "❌ Flap is disconnected"
        );
    }
}

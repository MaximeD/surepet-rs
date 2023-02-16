use std::vec;

use serde::{Deserialize, Serialize};

use crate::api::get_resources;
use crate::entities::{flap::Flap, hub::Hub, shared::Information};

const DEVICES_PATH: &str = "/api/device?with=status";

#[derive(Serialize, Deserialize, Debug)]
struct DevicesResponse {
    data: Vec<DeviceData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceData {
    name: String,
    product_id: u8, // Internal ID used to distinguish between the hub, a flap, …
    status: DeviceStatusData,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceStatusData {
    #[serde(skip_serializing_if = "Option::is_none")]
    battery: Option<f64>,
    online: bool,
}

/// Retrieve devices from api and convert them to the proper struct.
pub async fn devices() -> Vec<Box<dyn Information>> {
    let mut result: Vec<Box<dyn Information>> = vec![];

    match get_resources(DEVICES_PATH)
        .await
        .json::<DevicesResponse>()
        .await
    {
        Ok(parsed) => {
            for device in parsed.data {
                match device.product_id {
                    1 => result.push(Box::new(Hub {
                        name: device.name,
                        online: device.status.online,
                    })),
                    3 => result.push(Box::new(Flap {
                        name: device.name,
                        online: device.status.online,
                        battery_voltage: device.status.battery.unwrap(),
                    })),
                    _ => panic!("This device is unknown!"),
                }
            }
        }
        Err(_) => panic!("Hm, the response didn't match the shape we expected."),
    }
    return result;
}

#[cfg(test)]
use mockito;

mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use mockito::mock;

    #[test]
    fn it_returns_devices_information() {
        let _m = mock("GET", DEVICES_PATH)
            .with_status(200)
            .with_body(
                r#"
                {
                    "data": [
                        {
                            "id": 123,
                            "product_id": 1,
                            "name": "Hub",
                            "status": {
                                "led_mode": 1,
                                "pairing_mode": 0,
                                "online": true
                            }
                        },
                        {
                            "id": 456,
                            "product_id": 3,
                            "name": "Flap",
                            "status": {
                                "battery": 5.6175,
                                "locking": {
                                    "mode": 0
                                },
                                "online": true
                            }
                        }
                    ]
                }
                "#,
            )
            .create();

        let expected: Vec<Box<dyn Information>> = vec![
            Box::new(Hub {
                name: "Hub".to_string(),
                online: true,
            }),
            Box::new(Flap {
                name: "Flap".to_string(),
                online: true,
                battery_voltage: 5.6175,
            }),
        ];

        assert_eq!(tokio_test::block_on(devices()), expected);
    }

    #[test]
    #[should_panic(expected = "This device is unknown!")]
    fn it_panics_when_device_is_unknown() {
        let _m = mock("GET", DEVICES_PATH)
            .with_status(200)
            .with_body(
                r#"
                {
                    "data": [
                        {
                            "id": 123,
                            "product_id": 0,
                            "name": "Unknown",
                            "status": {
                                "online": true
                            }
                        }
                    ]
                }
                "#,
            )
            .create();

        tokio_test::block_on(devices());
    }

    #[test]
    #[should_panic(expected = "Uh oh! Something unexpected happened.")]
    fn it_panics_when_response_is_not_handled() {
        let _m = mock("GET", DEVICES_PATH).with_status(500).create();
        tokio_test::block_on(devices());
    }
}

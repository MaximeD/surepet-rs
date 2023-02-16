use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::get_resources;
use crate::entities::pet::Pet;

const PETS_PATH: &str = "/api/pet?with=position";

#[derive(Serialize, Deserialize, Debug)]
struct PetsResponse {
    data: Vec<PetData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PetData {
    name: String,
    position: PetPositionData,
}

#[derive(Serialize, Deserialize, Debug)]
struct PetPositionData {
    since: String,
    r#where: u8,
}

/// Retrieve pets from api and convert them to the proper struct.
pub async fn pets() -> Vec<Pet> {
    let mut pets: Vec<Pet> = vec![];

    match get_resources(PETS_PATH).await.json::<PetsResponse>().await {
        Ok(parsed) => {
            for pet in parsed.data {
                pets.push(Pet {
                    name: pet.name,
                    position: pet.position.r#where,
                    position_since: pet.position.since.parse::<DateTime<FixedOffset>>().unwrap(),
                });
            }
        }
        Err(_) => panic!("Hm, the response didn't match the shape we expected."),
    };
    return pets;
}

#[cfg(test)]
use mockito;

mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use mockito::mock;

    #[test]
    fn it_returns_pet_information() {
        let _m = mock("GET", PETS_PATH)
            .with_status(200)
            .with_body(
                r#"
                {
                    "data": [
                        {
                            "name": "Arlene",
                            "position": {
                                "where": 1,
                                "since": "2023-02-05T14:12:57+00:00"
                            }
                        },
                        {
                            "name": "Garfield",
                            "position": {
                                "where": 2,
                                "since": "2023-02-05T16:09:52+00:00"
                            }
                        }
                    ]
                }
                "#,
            )
            .create();
        assert_eq!(
            tokio_test::block_on(pets()),
            vec![
                Pet {
                    name: "Arlene".to_string(),
                    position: 1,
                    position_since: "2023-02-05T14:12:57+00:00"
                        .parse::<DateTime<FixedOffset>>()
                        .unwrap()
                },
                Pet {
                    name: "Garfield".to_string(),
                    position: 2,
                    position_since: "2023-02-05T16:09:52+00:00"
                        .parse::<DateTime<FixedOffset>>()
                        .unwrap()
                }
            ]
        )
    }

    #[test]
    #[should_panic(expected = "Uh oh! Something unexpected happened.")]
    fn it_panics_when_response_is_not_handled() {
        let _m = mock("GET", PETS_PATH).with_status(500).create();
        tokio_test::block_on(pets());
    }
}

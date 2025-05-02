use reqwest::Response;

pub mod devices;
pub mod login;
pub mod pets;

#[cfg(not(test))]
/// The base url of surepet API.
const BASE_URL: &str = "https://app.api.surehub.io";

#[cfg(not(test))]
use crate::utils::authentication::bearer_token;

/// Make an authenticated HTTP GET to the API.
#[cfg(not(test))]
async fn get_resources(path: &str) -> Response {
    let bearer_token = bearer_token().await;
    let url = format!("{}{}", BASE_URL, path);

    let client = reqwest::Client::builder()
        .user_agent("surepet-cli")
        .build()
        .unwrap();
    let response = client
        .get(url)
        .header("AUTHORIZATION", format!("Bearer {}", bearer_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => return response,
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    }
}

#[cfg(test)]
async fn get_resources(path: &str) -> Response {
    let bearer_token = "some_token";
    let url = format!("{}{}", std::env::var("MOCKITO_URL").unwrap(), path);

    let client = reqwest::Client::builder()
        .user_agent("surepet-cli")
        .build()
        .unwrap();
    let response = client
        .get(url)
        .header("AUTHORIZATION", format!("Bearer {}", bearer_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => return response,
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    }
}

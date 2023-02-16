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
async fn get_resources(path: &str) -> Response {
    #[cfg(not(test))]
    let bearer_token = bearer_token().await;

    #[cfg(test)]
    let bearer_token = "some_token";

    #[cfg(not(test))]
    let url = format!("{}{}", BASE_URL, path);

    #[cfg(test)]
    let url = format!("{}{}", &mockito::server_url(), path);

    let client = reqwest::Client::new();
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

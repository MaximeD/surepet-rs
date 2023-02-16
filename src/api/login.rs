use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[cfg(not(test))]
use crate::api::BASE_URL;

const LOGIN_PATH: &str = "/api/auth/login";

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    data: LoginData,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    token: String,
}

/// Get an authentication token.
///
/// It expects the following environment variables to be defined:
///
/// - `SUREPET_EMAIL`: email of your surepet account
/// - `SUREPET_PASSWORD`: password of your surepet account
pub async fn login() -> String {
    let email = env::var("SUREPET_EMAIL").expect("Please set `SUREPET_EMAIL` env variable");
    let password =
        env::var("SUREPET_PASSWORD").expect("Please set `SUREPET_PASSWORD` env variable");
    let mut payload = HashMap::new();
    payload.insert("email_address", email);
    payload.insert("password", password);
    payload.insert("device_id", ".".to_string());

    #[cfg(not(test))]
    let url = format!("{}{}", BASE_URL, LOGIN_PATH);

    #[cfg(test)]
    let url = format!("{}{}", &mockito::server_url(), LOGIN_PATH);

    let client = reqwest::Client::new();
    let response = client.post(url).json(&payload).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<LoginResponse>().await {
                Ok(parsed) => return parsed.data.token,
                Err(_) => panic!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Invalid credentials");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    }
}

#[cfg(test)]
use mockito;

mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use mockito::mock;

    #[test]
    fn it_returns_the_token() {
        let _m = mock("POST", LOGIN_PATH)
            .with_status(200)
            .with_body(r#"{"data": {"token": "some_token"}}"#)
            .create();
        env::set_var("SUREPET_EMAIL", "some_email@example.com");
        env::set_var("SUREPET_PASSWORD", "password");
        assert_eq!(tokio_test::block_on(login()), "some_token");
    }

    #[test]
    #[should_panic(expected = "Please set `SUREPET_EMAIL` env variable")]
    fn it_panics_when_environment_variables_are_missing() {
        env::remove_var("SUREPET_EMAIL");
        tokio_test::block_on(login());
    }

    #[test]
    #[should_panic(expected = "Invalid credentials")]
    fn it_panics_when_credentials_are_invalid() {
        let _m = mock("POST", LOGIN_PATH).with_status(401).create();
        env::set_var("SUREPET_EMAIL", "some_email@example.com");
        env::set_var("SUREPET_PASSWORD", "password");
        tokio_test::block_on(login());
    }

    #[test]
    #[should_panic(expected = "Uh oh! Something unexpected happened.")]
    fn it_panics_when_response_is_not_handled() {
        let _m = mock("POST", LOGIN_PATH).with_status(500).create();
        env::set_var("SUREPET_EMAIL", "some_email@example.com");
        env::set_var("SUREPET_PASSWORD", "invalid password");
        tokio_test::block_on(login());
    }
}

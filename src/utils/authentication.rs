#[cfg(not(test))]
use std::fs;

#[cfg(not(test))]
use crate::api::login::login;

#[cfg(not(test))]
const TOKEN_PATH: &str = ".surepet/token";

#[cfg(not(test))]
/// Read token from file, or query API to get a new one.
pub async fn bearer_token() -> String {
    match home::home_dir() {
        Some(home) => {
            let full_path = format!("/{}/{}", home.display(), TOKEN_PATH);
            let token_path = std::path::Path::new(&full_path);

            if token_path.exists() {
                match fs::read_to_string(token_path) {
                    Ok(token) => return token,
                    _ => panic!("Cannot read token from file"),
                }
            } else {
                let token = login().await;

                fs::create_dir_all(token_path.parent().unwrap()).unwrap();
                fs::write(token_path, &token).expect("Unable to write file");
                return token;
            }
        }
        None => panic!("Impossible to get your home dir!"),
    }
}

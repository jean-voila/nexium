use super::login;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

const GITLAB_API_URL: &str = "https://gitlab.cri.epita.fr/api/v4";

#[derive(Debug)]
pub struct GitlabToken {
    token: String,
    login: login::Login,
}

impl GitlabToken {
    pub fn new(token: String, login: login::Login) -> GitlabToken {
        GitlabToken { token, login }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn check_token(&self) -> bool {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );

        let url = format!("{}/users?username={}", GITLAB_API_URL, self.login);
        let res = client
            .get(&url)
            .headers(headers)
            .send()
            .expect("Error sending request to the gitlab api");

        return res.status().is_success();
    }
}

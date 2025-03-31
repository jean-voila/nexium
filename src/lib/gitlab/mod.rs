use reqwest::blocking::Client;
use serde_json;
#[derive(Debug)]
pub struct GitlabClient {
    api_url: String,
    token: String,
}

#[derive(Debug)]
pub enum GitlabError {
    InvalidToken,
    NetworkError,
    UserNotFound,
    UnknownError,
    NoGPGKeys,
    BadGPGFormat,
}

impl GitlabClient {
    pub fn new(api_url: String, token: String) -> Self {
        GitlabClient { api_url, token }
    }
    pub fn check_token(&self) -> Result<bool, GitlabError> {
        let url = format!("{}/user", self.api_url);
        let client = Client::new();
        let response =
            client.get(&url).header("PRIVATE-TOKEN", &self.token).send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Ok(true);
                }
                return Err(GitlabError::InvalidToken);
            }
            Err(_) => return Err(GitlabError::NetworkError),
        }
    }

    pub fn get_gpg_keys(
        &self,
        login: &str,
    ) -> Result<Vec<String>, GitlabError> {
        let user_id = self.get_user_id(login);
        match user_id {
            Ok(id) => {
                let gpg_keys = self.get_user_gpg_keys_by_id(id);
                match gpg_keys {
                    Ok(keys) => {
                        if keys.is_empty() {
                            return Err(GitlabError::NoGPGKeys);
                        }
                        return Ok(keys);
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn get_user_id(&self, login: &str) -> Result<u64, GitlabError> {
        let url = format!("{}/users", self.api_url);
        let client = Client::new();
        let response = client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.token)
            .query(&[("username", login)])
            .send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let users: Vec<serde_json::Value> =
                        resp.json().unwrap_or(Vec::new());
                    if users.is_empty() {
                        return Err(GitlabError::UserNotFound);
                    }
                    if let Some(user) = users.get(0) {
                        if let Some(id) = user.get("id") {
                            if let Some(id) = id.as_u64() {
                                return Ok(id);
                            }
                        }
                    }
                }
                Err(GitlabError::InvalidToken)
            }
            Err(_) => Err(GitlabError::NetworkError),
        }
    }

    fn get_user_gpg_keys_by_id(
        &self,
        user_id: u64,
    ) -> Result<Vec<String>, GitlabError> {
        let url = format!("{}/users/{}/gpg_keys", self.api_url, user_id);
        let client = Client::new();
        let response =
            client.get(&url).header("PRIVATE-TOKEN", &self.token).send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let keys: Vec<serde_json::Value> =
                        resp.json().unwrap_or(Vec::new());
                    let mut gpg_keys: Vec<String> = Vec::new();
                    for key in keys {
                        if let Some(key) = key.get("key") {
                            if let Some(key) = key.as_str() {
                                gpg_keys.push(key.to_string());
                            }
                        }
                    }
                    return Ok(gpg_keys);
                }
                Err(GitlabError::InvalidToken)
            }
            Err(_) => Err(GitlabError::NetworkError),
        }
    }

    pub fn add_gpg_key(&self, gpg_key: &str) -> Result<(), GitlabError> {
        let url = format!("{}/user/gpg_keys", self.api_url);

        let client = reqwest::blocking::Client::new();

        let response = client
            .post(&url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "key": gpg_key
            }))
            .send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Ok(());
                } else if resp.status().as_u16() == 400 {
                    dbg!(resp.text().unwrap());
                    return Err(GitlabError::BadGPGFormat);
                }
                return Err(GitlabError::InvalidToken);
            }
            Err(_) => return Err(GitlabError::NetworkError),
        }
    }
}

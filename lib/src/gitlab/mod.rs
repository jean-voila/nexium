use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::blocking::Client;
use serde_json;

use super::defaults::*;

use core::fmt;
use std::io::{BufRead, Write};
use std::net::TcpListener;
use url::Url;
use webbrowser;

/// Default path to the Gitlab API URL

#[derive(Default, Debug)]
pub enum TokenType {
    #[default]
    Classic,
    OAuth,
}

#[derive(Debug)]
pub struct GitlabClient {
    api_url: String,
    token: String,
    token_type: TokenType,
}

#[derive(Debug)]
pub enum GitlabError {
    InvalidToken,
    NetworkError,
    UserNotFound,
    UnknownError,
    BadGPGFormat,
    NoWebBrowser,
    AbortedLogin,
}

impl fmt::Display for GitlabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitlabError::InvalidToken => write!(f, "Invalid token"),
            GitlabError::NetworkError => write!(f, "Network error"),
            GitlabError::UserNotFound => write!(f, "User not found"),
            GitlabError::UnknownError => write!(f, "Unknown error"),
            GitlabError::BadGPGFormat => write!(f, "Bad GPG format"),
            GitlabError::NoWebBrowser => write!(f, "No web browser found"),
            GitlabError::AbortedLogin => write!(f, "Login aborted"),
        }
    }
}

impl GitlabClient {
    pub fn new(token: String, token_type: TokenType) -> Self {
        GitlabClient {
            api_url: format!("{}/api/v4", GITLAB_URL),
            token,
            token_type,
        }
    }
    pub fn check_token(&self) -> Result<bool, GitlabError> {
        let url = format!("{}/user", self.api_url);
        let client = Client::new();
        let request = client.get(&url);

        let request = match self.token_type {
            TokenType::Classic => request.header("PRIVATE-TOKEN", &self.token),
            TokenType::OAuth => request
                .header("Authorization", format!("Bearer {}", &self.token)),
        };

        let response = request.send();

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(true)
                } else {
                    Err(GitlabError::InvalidToken)
                }
            }
            Err(_) => Err(GitlabError::NetworkError),
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
        let request = client.get(&url).query(&[("username", login)]);
        let request = match self.token_type {
            TokenType::Classic => request.header("PRIVATE-TOKEN", &self.token),
            TokenType::OAuth => request
                .header("Authorization", format!("Bearer {}", &self.token)),
        };
        let response = request.send();

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
        let request = client.get(&url);

        let request = match self.token_type {
            TokenType::Classic => request.header("PRIVATE-TOKEN", &self.token),
            TokenType::OAuth => request
                .header("Authorization", format!("Bearer {}", &self.token)),
        };

        let response = request.send();

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
                    Ok(gpg_keys)
                } else {
                    Err(GitlabError::InvalidToken)
                }
            }
            Err(_) => Err(GitlabError::NetworkError),
        }
    }

    pub fn add_gpg_key(&self, gpg_key: &str) -> Result<(), GitlabError> {
        let url = format!("{}/user/gpg_keys", self.api_url);

        let client = reqwest::blocking::Client::new();

        let request = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "key": gpg_key
            }));
        let request = match self.token_type {
            TokenType::Classic => request.header("PRIVATE-TOKEN", &self.token),
            TokenType::OAuth => request
                .header("Authorization", format!("Bearer {}", &self.token)),
        };
        let response = request.send();

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

    pub fn get_token() -> Result<String, GitlabError> {
        // Configuration du client OAuth2
        let auth_url =
            AuthUrl::new(format!("{}/oauth/authorize", GITLAB_URL)).unwrap();
        let token_url =
            TokenUrl::new(format!("{}/oauth/token", GITLAB_URL)).unwrap();

        let client_id = ClientId::new(CLIENT_ID.to_string());
        let client_secret = ClientSecret::new(CLIENT_SECRET.to_string());
        let redirect_uri = RedirectUrl::new(REDIRECT_URI.to_string()).unwrap();

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_uri);

        // Génération du challenge PKCE
        let (pkce_challenge, pkce_verifier) =
            PkceCodeChallenge::new_random_sha256();

        // Génération de l'URL d'autorisation
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("api".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        // Ouverture de l'URL dans le navigateur par défaut
        let _ = open_authorization_url(&authorize_url.to_string());

        // Démarrage d'un serveur local pour écouter la redirection
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        let (code, state) = match listen_for_code(&listener) {
            Ok((code, state)) => (code, state),
            Err(_) => {
                return Err(GitlabError::AbortedLogin);
            }
        };

        // Vérification de l'état CSRF
        if &state != csrf_state.secret() {
            return Err(GitlabError::InvalidToken);
        }

        // Échange du code d'autorisation contre un token d'accès
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request(&Client::new())
            .unwrap();

        Ok(token_result.access_token().secret().clone())
    }
}

fn open_authorization_url(auth_url: &str) -> Result<(), GitlabError> {
    match webbrowser::open(auth_url) {
        Ok(_) => Ok(()),
        Err(_) => Err(GitlabError::NoWebBrowser),
    }
}

fn listen_for_code(
    listener: &TcpListener,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = std::io::BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        // Extraction du code d'autorisation et de l'état CSRF de l'URL
        if let Some(code) = request_line.split_whitespace().nth(1) {
            let url = Url::parse(&format!("http://localhost{}", code))?;
            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned())
                .ok_or("Code d'autorisation non trouvé")?;
            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned())
                .ok_or("État CSRF non trouvé")?;

            // Réponse HTTP simple pour indiquer la réussite
            let response = "HTTP/1.1 200 OK\r\n\r\nYour Nexium app is successfully logged to Gitlab.";

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
            return Ok((code, state));
        }
    }
    Err("Impossible d'obtenir le code d'autorisation".into())
}

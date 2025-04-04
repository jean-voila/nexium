use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::blocking::Client;
use serde_json;

use std::io::{BufRead, Write};
use std::net::TcpListener;

use url::Url;
use webbrowser;

/// Default path to the Gitlab API URL
const GITLAB_URL: &str = "https://gitlab.cri.epita.fr";

const CLIENT_ID: &str =
    "f180d1cbd126017dcc20629aee0af5dd229dc5fd13d19c6a9ace1361e2039c59";
const CLIENT_SECRET: &str =
    "gloas-83e39025a820061c0744402c93a5915c5dc1bdb30ac2d23df3b68485b044545c";
const REDIRECT_URI: &str = "http://localhost:8080/callback";

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
    NoWebBrowser,
}

impl GitlabClient {
    pub fn new(token: String) -> Self {
        GitlabClient {
            api_url: format!("{}/api/v4", GITLAB_URL),
            token,
        }
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
        let (code, state) = listen_for_code(&listener).unwrap();

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
            let response = "HTTP/1.1 200 OK\r\n\r\nAuthentification réussie. Vous pouvez fermer cette fenêtre.";

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
            return Ok((code, state));
        }
    }
    Err("Impossible d'obtenir le code d'autorisation".into())
}

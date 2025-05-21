use reqwest::Client;

pub struct NexiumClient {
    client: Client,
    login: String,
    server_url: String,
    server_port: String,
}

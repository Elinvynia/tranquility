//! The basic authentication method, using a reddit username and password.

use crate::{auth::Auth, client::route::Route, error::Error};
use async_trait::async_trait;
use reqwest::{Client as HttpClient, Response};

/// The basic authentication method for Reddit bots.
/// It requires the use of the "script" account type.
#[derive(Debug)]
pub struct BasicAuth {
    /// The application ID.
    pub client_id: String,
    /// The secret OAuth application key.
    pub secret_key: String,
    /// The username of the bot account.
    pub username: String,
    /// The password of the bot account.
    pub password: String,
    http_client: HttpClient,
}

impl BasicAuth {
    /// Builder for the BasicAuth method.
    pub async fn new(
        client_id: String,
        secret_key: String,
        username: String,
        password: String,
    ) -> Self {
        BasicAuth {
            client_id,
            secret_key,
            username,
            password,
            http_client: HttpClient::new(),
        }
    }
}

#[async_trait]
impl Auth for BasicAuth {
    async fn login(&self) -> Result<String, Error> {
        let response = self
            .http_client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(&self.client_id, Some(&self.secret_key))
            .body(format!(
                "grant_type=password&username={}&password={}",
                self.username, self.password
            ))
            .send()
            .await?
            .text()
            .await?;

        let json: serde_json::Value = serde_json::from_str(&response)?;
        let map = json.as_object().ok_or_else(|| "Bad response")?;

        if let Some(e) = map.get("error") {
            return Err(Error::Custom(format!(
                "Error {}: {}",
                e,
                map.get("message").unwrap()
            )));
        };

        let token = map
            .get("access_token")
            .unwrap()
            .to_string()
            .replace(r#"""#, "");

        Ok(token)
    }

    async fn get(&self, route: Route, key: &str, user_agent: &str) -> Result<Response, Error> {
        let request = self
            .http_client
            .get(&route.to_string())
            .header("User-Agent", user_agent)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }
}

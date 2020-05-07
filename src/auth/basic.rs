//! The basic authentication method, using a reddit username and password.

use crate::{auth::Auth, client::route::Route, error::Error};
use async_trait::async_trait;
use reqwest::{Client as HttpClient, Response};
use std::time::{Duration, SystemTime};

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
    pub(crate) expiration: Duration,
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
            expiration: Duration::from_secs(3600),
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

        let token: String = map
            .get("access_token")
            .ok_or_else(|| "Authentication failed")?
            .as_str()
            .ok_or_else(|| "Authentication failed")?
            .to_string();

        Ok(token)
    }

    async fn get(&self, route: Route, key: &str, user_agent: &str) -> Result<Response, Error> {
        if SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Your system time is before Linux Epoch")
            > self.expiration
        {
            self.login().await?;
        };
        let request = self
            .http_client
            .get(&format!("{}{}", route.to_string(), "?raw_json=1"))
            .header("User-Agent", user_agent)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }
}

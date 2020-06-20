//! The basic authentication method, using a reddit username and password.

use crate::error::ApiError;
use crate::{auth::Auth, client::route::Route, error::Error, model::misc::Params};
use async_trait::async_trait;
use reqwest::{Client as HttpClient, Response};
use std::sync::{Arc, RwLock};
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
    pub(crate) expiration: Arc<RwLock<Duration>>,
    http_client: HttpClient,
}

impl BasicAuth {
    /// Builder for the BasicAuth method.
    pub async fn new(client_id: &str, secret_key: &str, username: &str, password: &str) -> Self {
        BasicAuth {
            client_id: client_id.to_string(),
            secret_key: secret_key.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            http_client: HttpClient::new(),
            expiration: Arc::new(RwLock::new(Duration::from_secs(3600))),
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

        let expiration: u64 = map
            .get("expires_in")
            .ok_or_else(|| "Authentication error: No `expires_in` field in response")?
            .as_u64()
            .ok_or_else(|| "`expires_in` field is an invalid type")?;

        {
            let mut exp = self
                .expiration
                .write()
                .map_err(|_| "Expiration lock is poisoned")?;
            *exp = Duration::from_secs(expiration);
        }

        let token: String = map
            .get("access_token")
            .ok_or_else(|| "Authentication error: No `access_token` field in response")?
            .as_str()
            .ok_or_else(|| "`access_token` field is an invalid type")?
            .to_string();

        Ok(token)
    }

    async fn get(
        &self,
        route: Route,
        key: &str,
        user_agent: &str,
        params: &Params,
    ) -> Result<Response, Error> {
        let mut login = false;
        {
            let exp = self
                .expiration
                .read()
                .map_err(|_| "Expiration lock is poisoned")?;
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Your system time is before Linux Epoch")
                > *exp
            {
                login = true;
            };
        }

        if login {
            self.login().await?;
        };

        let request = self
            .http_client
            .get(&format!("{}{}", route.to_string(), "?raw_json=1"))
            .header("User-Agent", user_agent)
            .query(&params)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }

    async fn post(
        &self,
        route: Route,
        key: &str,
        user_agent: &str,
        params: &Params,
    ) -> Result<Response, Error> {
        let mut login = false;
        {
            let exp = self
                .expiration
                .read()
                .map_err(|_| "Expiration lock is poisoned")?;
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Your system time is before Linux Epoch")
                > *exp
            {
                login = true;
            };
        }

        if login {
            self.login().await?;
        };

        let request = self
            .http_client
            .post(&format!("{}{}", route.to_string(), "?raw_json=1"))
            .header("User-Agent", user_agent)
            .query(&params)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }

    async fn delete(&self, route: Route, key: &str, user_agent: &str) -> Result<Response, Error> {
        let mut login = false;
        {
            let exp = self
                .expiration
                .read()
                .map_err(|_| "Expiration lock is poisoned")?;
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Your system time is before Linux Epoch")
                > *exp
            {
                login = true;
            };
        }

        if login {
            self.login().await?;
        };

        let request = self
            .http_client
            .delete(&format!("{}{}", route.to_string(), "?raw_json=1"))
            .header("User-Agent", user_agent)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }

    async fn put(
        &self,
        route: Route,
        key: &str,
        user_agent: &str,
        params: &Params,
    ) -> Result<Response, Error> {
        let mut login = false;
        {
            let exp = self
                .expiration
                .read()
                .map_err(|_| "Expiration lock is poisoned")?;
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Your system time is before Linux Epoch")
                > *exp
            {
                login = true;
            };
        }

        if login {
            self.login().await?;
        };

        let request = self
            .http_client
            .put(&format!("{}{}", route.to_string(), "?raw_json=1"))
            .header("User-Agent", user_agent)
            .query(&params)
            .bearer_auth(key);

        let response = request.send().await?;

        Ok(response)
    }
}

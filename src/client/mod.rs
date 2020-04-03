use crate::error::Error;
use crate::model::me::Me;
use async_trait::async_trait;
use reqwest::Client as HttpClient;

pub struct Client<T: Auth> {
    auth: T,
    http_client: HttpClient,
    access_token: String,
}

impl<T: Auth + Send + Sync> Client<T> {
    pub async fn new(auth_method: T) -> Self {
        let token = auth_method.login().await.unwrap();
        let http = HttpClient::new();
        Client {
            auth: auth_method,
            access_token: token,
            http_client: http,
        }
    }

    pub async fn me(&self) -> Result<Me, Error> {
        let response = self
            .auth
            .get("/api/v1/me", &self.access_token)
            .await
            .unwrap();
        let me: Me = serde_json::from_str(&response)?;
        Ok(me)
    }
}

#[async_trait]
pub trait Auth {
    async fn login(&self) -> Result<String, Error> {
        Ok("".to_string())
    }

    async fn get(&self, route: &str, key: &str) -> Result<String, Error> {
        Ok("".to_string())
    }
}

pub struct BasicAuth {
    pub client_id: String,
    pub secret_key: String,
    pub username: String,
    pub password: String,
    pub http_client: HttpClient,
}

impl BasicAuth {
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
        let url = "https://www.reddit.com/api/v1/access_token";
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

    async fn get(&self, route: &str, key: &str) -> Result<String, Error> {
        let url = format!("https://oauth.reddit.com{}", route);
        let request = self
            .http_client
            .get(&url)
            .header("User-Agent", "Tranquility by tranquility-rs")
            .bearer_auth(key);

        let response = request.send().await?.text().await?;

        Ok(response)
    }
}

pub mod route;

use crate::{auth::Auth, error::Error};
use reqwest::Response;

pub struct Client<T: Auth> {
    access_token: String,
    auth: T,
    user_agent: String,
}

impl<T: Auth + Send + Sync> Client<T> {
    pub async fn new(user_agent: &str, auth_method: T) -> Result<Self, Error> {
        let access_token = auth_method.login().await?;
        let ua = user_agent.to_string();
        Ok(Client {
            auth: auth_method,
            access_token,
            user_agent: ua,
        })
    }

    pub async fn get(&self, route: &str) -> Result<Response, Error> {
        self.auth
            .get(&route.to_string(), &self.access_token, &self.user_agent)
            .await
    }
}

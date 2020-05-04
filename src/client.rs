//! Client is used to handle requests through the reddit API.

pub(crate) mod route;

use crate::{
    auth::Auth,
    client::route::Route,
    error::Error,
    model::{thing::unwrap_thing, user::User},
};
use reqwest::Response;

/// The client handling the requests.
#[derive(Debug)]
pub struct Client<T: Auth> {
    /// Access token used for all requests.
    access_token: String,
    /// The auth model chosen for connecting to reddit.
    auth: T,
    /// User agent representing the client sending the requests.
    user_agent: String,
}

impl<T: Auth + Send + Sync> Client<T> {
    /// The function used to construct a new client.
    pub async fn new(user_agent: &str, auth_method: T) -> Result<Self, Error> {
        let access_token = auth_method.login().await?;
        let ua = user_agent.to_string();
        Ok(Client {
            auth: auth_method,
            access_token,
            user_agent: ua,
        })
    }

    async fn get(&self, route: Route) -> Result<Response, Error> {
        self.auth
            .get(route, &self.access_token, &self.user_agent)
            .await
    }

    /// Retrieves the user information given a username.
    pub async fn user(&self, username: &str) -> Result<User, Error> {
        let response = self.get(Route::UserAbout(username.to_string())).await?;
        let body = response.text().await?;
        let map = unwrap_thing(&body)?;
        let user: User = serde_json::from_str(&map)?;
        Ok(user)
    }
}

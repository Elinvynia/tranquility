//! Client is used to handle requests through the reddit API.

pub(crate) mod route;

use crate::{
    auth::Auth,
    client::route::Route,
    error::Error,
    model::{link::Link, listing::Listing, subreddit::Subreddit, thing::Thing, user::User},
};
use futures_timer::Delay;
use reqwest::Response;
use std::convert::TryInto;
use std::time::Duration;

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
        Ok(Client {
            auth: auth_method,
            access_token,
            user_agent: user_agent.to_string(),
        })
    }

    async fn get(&self, route: Route) -> Result<Response, Error> {
        let response = self
            .auth
            .get(route, &self.access_token, &self.user_agent)
            .await?;

        self.handle_ratelimit(&response).await?;
        Ok(response)
    }

    async fn handle_ratelimit(&self, response: &Response) -> Result<(), Error> {
        let headers = response.headers();
        let _used: u64 = headers
            .get("x-ratelimit-used")
            .ok_or_else(|| Error::MissingHeader("x-ratelimit-used".to_string()))?
            .to_str()?
            .parse()?;

        let remaining: u64 = headers
            .get("x-ratelimit-remaining")
            .ok_or_else(|| Error::MissingHeader("x-ratelimit-remaining".to_string()))?
            .to_str()?
            .parse::<f64>()?
            .to_bits();

        let reset: u64 = headers
            .get("x-ratelimit-reset")
            .ok_or_else(|| Error::MissingHeader("x-ratelimit-reset".to_string()))?
            .to_str()?
            .parse()?;

        if remaining < 1 {
            let _ = Delay::new(Duration::from_secs(reset)).await;
        }

        Ok(())
    }

    /// Retrieves the user information given a username.
    pub async fn user(&self, username: &str) -> Result<User, Error> {
        let response = self.get(Route::UserAbout(username.to_string())).await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let user: User = Thing::try_into(thing)?;
        Ok(user)
    }

    /// Retrieves the subreddit information given the name.
    pub async fn subreddit(&self, subreddit: &str) -> Result<Subreddit, Error> {
        let response = self
            .get(Route::SubredditAbout(subreddit.to_string()))
            .await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let user: Subreddit = Thing::try_into(thing)?;
        Ok(user)
    }

    pub(crate) async fn get_posts(&self, route: Route) -> Result<Vec<Link>, Error> {
        let response = self.get(route).await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let listing: Listing = Thing::try_into(thing)?;
        let mut links: Vec<Link> = Vec::new();
        if let Some(c) = &listing.children {
            for x in c {
                let y = x.clone();
                let link: Link = Thing::try_into(y)?;
                links.push(link);
            }
        }
        Ok(links)
    }
}

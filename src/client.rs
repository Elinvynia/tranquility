//! Client is used to handle requests through the reddit API.

pub(crate) mod route;

use crate::{
    auth::Auth,
    client::route::Route,
    error::Error,
    model::misc::Params,
    model::{
        comment::Comment, link::Link, listing::Listing, subreddit::Subreddit, thing::Thing,
        user::User,
    },
};
use futures_timer::Delay;
use reqwest::Response;
use serde_json::{Map, Value};
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

    /// Retrieves the user information given a username.
    pub async fn user(&self, username: &str) -> Result<User, Error> {
        let response = self
            .get(Route::UserAbout(username.into()), &Params::new())
            .await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let user: User = Thing::try_into(thing)?;
        Ok(user)
    }

    /// Retrieves the subreddit information given the name.
    pub async fn subreddit(&self, subreddit: &str) -> Result<Subreddit, Error> {
        let response = self
            .get(Route::SubredditAbout(subreddit.into()), &Params::new())
            .await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let user: Subreddit = Thing::try_into(thing)?;
        Ok(user)
    }

    /// Returns the comment data from its ID.
    pub async fn comment(&self, comment: &str) -> Result<Comment, Error> {
        let response = self
            .get(
                Route::Info,
                &Params::new().add("id", &format!("t1_{}", comment)),
            )
            .await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let listing: Listing = Thing::try_into(thing)?;
        let mut comments: Vec<Comment> = Listing::try_into(listing)?;
        let comment = comments.remove(0);
        Ok(comment)
    }

    /// Returns the link data from its ID.
    pub async fn link(&self, link: &str) -> Result<Link, Error> {
        let response = self
            .get(
                Route::Info,
                &Params::new().add("id", &format!("t3_{}", link)),
            )
            .await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let listing: Listing = Thing::try_into(thing)?;
        let mut links: Vec<Link> = Listing::try_into(listing)?;
        let link = links.remove(0);
        Ok(link)
    }

    /// Returns the link data from its ID.
    pub async fn username_available(&self, username: &str) -> Result<bool, Error> {
        let response = self
            .get(
                Route::UsernameAvailable,
                &Params::new().add("user", username),
            )
            .await?;
        let body = response.text().await?;
        let available: bool = serde_json::from_str(&body)?;
        Ok(available)
    }

    pub(crate) async fn get(&self, route: Route, params: &Params) -> Result<Response, Error> {
        let response = self
            .auth
            .get(route, &self.access_token, &self.user_agent, params)
            .await?;

        self.handle_ratelimit(&response).await?;
        Ok(response)
    }

    pub(crate) async fn post(&self, route: Route, params: &Params) -> Result<Response, Error> {
        let response = self
            .auth
            .post(route, &self.access_token, &self.user_agent, params)
            .await?;

        self.handle_ratelimit(&response).await?;
        Ok(response)
    }

    pub(crate) async fn delete(&self, route: Route) -> Result<Response, Error> {
        let response = self
            .auth
            .delete(route, &self.access_token, &self.user_agent)
            .await?;

        self.handle_ratelimit(&response).await?;
        Ok(response)
    }

    pub(crate) async fn put(&self, route: Route, params: &Params) -> Result<Response, Error> {
        let response = self
            .auth
            .put(route, &self.access_token, &self.user_agent, params)
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

    pub(crate) async fn get_posts(&self, route: Route) -> Result<Vec<Link>, Error> {
        let response = self.get(route, &Params::new()).await?;
        let body = response.text().await?;
        let thing: Thing = serde_json::from_str(&body)?;
        let listing: Listing = Thing::try_into(thing)?;
        let mut links: Vec<Link> = Vec::new();
        for x in &listing.children {
            let y = x.clone();
            let link: Link = Thing::try_into(y)?;
            links.push(link);
        }
        Ok(links)
    }

    pub(crate) async fn replies(
        &self,
        link_id: String,
        name: String,
    ) -> Result<Vec<Comment>, Error> {
        let path = format!("/comments/{}/_/{}", link_id, name);
        let params = Params::new().add("context", "0").add("limit", "100");
        let response = self.get(Route::Custom(path), &params).await?;
        let body = response.text().await?;
        let mut listings: Vec<Thing> = serde_json::from_str(&body)?;
        let listing: Listing = Thing::try_into(listings.remove(1))?;
        let mut comments: Vec<Comment> = Listing::try_into(listing)?;
        let comment: Comment = comments.remove(0);
        let replies: Thing = *comment
            .replies
            .ok_or_else(|| Error::Custom("No replies".into()))?;
        let replies2: Listing = Thing::try_into(replies)?;
        let replies3: Vec<Comment> = Listing::try_into(replies2)?;
        Ok(replies3)
    }

    pub(crate) async fn submit_comment(&self, thing_id: &str, body: &str) -> Result<(), Error> {
        let response = self
            .post(
                Route::Comment,
                &Params::new()
                    .add("thing_id", thing_id)
                    .add("text", body)
                    .add("api_type", "json"),
            )
            .await?;
        let body = response.text().await?;
        let parsed: Map<String, Value> = serde_json::from_str(&body)?;
        let json = parsed
            .get("json")
            .ok_or_else(|| "Invalid response 1")?
            .as_object()
            .ok_or_else(|| "Invalid response 2")?;
        let errors: Vec<Value> = json
            .get("errors")
            .ok_or_else(|| "Invalid response 3")?
            .as_array()
            .ok_or_else(|| "Invalid response 4")?
            .clone();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Custom("Invalid response 5".into()))
        }
    }
}

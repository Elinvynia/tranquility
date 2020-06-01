//! Module related to the user struct.

use crate::{
    auth::Auth,
    client::{route::Route, Client},
    error::Error,
    model::{misc::Fullname, misc::Params, usersubreddit::UserSubreddit},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// The struct representing a reddit user.
pub struct User {
    /// The comment karma of the user.
    pub comment_karma: i64,
    /// When the user was created, local timezone.
    pub created: f64,
    /// When the user was created, normalized to UTC.
    pub created_utc: f64,
    /// If the user has a verified email.
    pub has_verified_email: bool,
    /// If the user should be hidden from robots.
    pub hide_from_robots: bool,
    /// Link to the image of the user's icon.
    pub icon_img: String,
    /// The API ID of the user.
    pub id: Fullname,
    /// If the user is an employee of Reddit.
    pub is_employee: bool,
    /// If the user is added as a friend.
    pub is_friend: bool,
    /// If the user currently has gold.
    pub is_gold: bool,
    /// If the user is a global moderator.
    pub is_mod: bool,
    /// The link karma of the user.
    pub link_karma: i64,
    /// The name of the user.
    pub name: String,
    /// The user's subreddit.
    pub subreddit: UserSubreddit,
    /// If the user is verified.
    pub verified: bool,
}

impl User {
    /// Reports the User to the reddit admins.
    pub async fn block<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(
                Route::BlockUser,
                &Params::new()
                    .add("name", &self.name)
                    .add("account_id", self.id.as_ref())
                    .add("api_type", "json"),
            )
            .await
            .and(Ok(()))
    }

    /// Adds the User to friends.
    pub async fn friend<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
        note: Option<&str>,
    ) -> Result<(), Error> {
        client
            .put(
                Route::Friends(self.name.clone()),
                &Params::new()
                    .add("name", &self.name)
                    .add("note", note.unwrap_or("")),
            )
            .await
            .and(Ok(()))
    }

    /// Removes a User from your friends.
    pub async fn unfriend<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .delete(Route::Friends(self.name.clone()))
            .await
            .and(Ok(()))
    }

    /// Reports the User to the reddit admins.
    pub async fn report<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        client
            .post(
                Route::ReportUser,
                &Params::new()
                    .add("user", &self.name)
                    .add("reason", reason.unwrap_or("")),
            )
            .await
            .and(Ok(()))
    }
}

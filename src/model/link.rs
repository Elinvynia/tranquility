//! A post on reddit.

use crate::{
    auth::Auth,
    client::{route::Route, Client},
    error::Error,
    model::{
        award::Award, comment::Comment, misc::Fullname, misc::Params, subreddit::Subreddit,
        user::User,
    },
};
use serde::{Deserialize, Serialize};

/// The struct representing a post on reddit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The title of the link.
    pub title: String,
    /// The username of the author who submitted the link.
    pub author: String,
    /// The score of this post, fuzzed.
    pub score: i64,
    /// The amount of comments.
    pub num_comments: u64,
    /// The name of this subreddit.
    pub subreddit: String,
    /// The fullname of this Link.
    pub name: Fullname,
    /// All awards belonging to this Link.
    pub all_awardings: Vec<Award>,
}

impl Link {
    /// Retrieves the User struct of the author.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }

    /// Retrieves the Subreddit struct where this Link was posted.
    pub async fn subreddit<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
    ) -> Result<Subreddit, Error> {
        client.subreddit(&self.subreddit).await
    }

    /// Returns the first-level replies to this post.
    pub async fn replies<T: Auth + Send + Sync>(
        &self,
        _client: &Client<T>,
    ) -> Result<Vec<Comment>, Error> {
        todo!()
    }

    /// Creates a top-level comment in this Link.
    pub async fn reply<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
        body: &str,
    ) -> Result<(), Error> {
        client.submit_comment(self.name.as_ref(), body).await
    }

    /// Spoilers the Link assuming you have the permission to do so.
    pub async fn spoiler<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::Spoiler, &Params::new().add("id", self.name.as_ref()))
            .await
            .and(Ok(()))
    }

    /// Unspoilers the Link assuming you have the permission to do so.
    pub async fn unspoiler<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(
                Route::Unspoiler,
                &Params::new().add("id", self.name.as_ref()),
            )
            .await
            .and(Ok(()))
    }

    /// Adds the NSFW mark to the Link assuming you have the permission to do so.
    pub async fn set_nsfw<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::SetNSFW, &Params::new().add("id", self.name.as_ref()))
            .await
            .and(Ok(()))
    }

    /// Removes the NSFW mark from the Link assuming you have the permission to do so.
    pub async fn unset_nsfw<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::UnsetNSFW, &Params::new().add("id", self.name.as_ref()))
            .await
            .and(Ok(()))
    }

    /// Locks the Link assuming you have the permission to do so.
    pub async fn lock<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::Lock, &Params::new().add("id", self.name.as_ref()))
            .await
            .and(Ok(()))
    }

    /// Unlocks the Link assuming you have the permission to do so.
    pub async fn unlock<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::Unlock, &Params::new().add("id", self.name.as_ref()))
            .await
            .and(Ok(()))
    }

    /// Start following this link.
    pub async fn follow<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::Follow, &Params::new().add("id", self.name.as_ref()).add("follow", "1"))
            .await
            .and(Ok(()))
    }

    /// Stop following this link.
    pub async fn unfollow<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<(), Error> {
        client
            .post(Route::Follow, &Params::new().add("id", self.name.as_ref()).add("follow", "0"))
            .await
            .and(Ok(()))
    }
}

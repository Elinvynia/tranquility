//! Contains the comment model.

use crate::{
    auth::Auth,
    client::Client,
    error::Error,
    model::{link::Link, listing::Listing, user::User},
};
use serde::{Deserialize, Serialize};

/// A comment that can be anywhere.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    /// The username of the author of this comment. If you want to get the User you should use the `author()` method.
    pub author: String,

    replies: Option<Listing>,
}

impl Comment {
    /// Returns the User that made this comment.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }

    /// Returns the replies to this comment only.
    pub async fn replies<T: Auth + Send + Sync>(
        &self,
        _client: &Client<T>,
    ) -> Result<Option<Vec<Comment>>, Error> {
        if self.replies.is_none() {
            return Ok(None);
        }
        todo!()
    }

    /// Returns the parent of this comment, if it exists.
    pub async fn parent<T: Auth + Send + Sync>(
        &self,
        _client: &Client<T>,
    ) -> Result<Option<Comment>, Error> {
        todo!()
    }

    /// Returns the link where this comment was made.
    pub async fn link<T: Auth + Send + Sync>(&self, _client: &Client<T>) -> Result<Link, Error> {
        todo!()
    }

    /// Send a reply to this comment.
    pub async fn reply<T: Auth + Send + Sync>(&self, _client: &Client<T>) -> Result<(), Error> {
        todo!()
    }
}

//! Contains the comment model.

use crate::{
    auth::Auth,
    client::Client,
    error::Error,
    model::{link::Link, misc::Fullname, user::User, thing::Thing},
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// A comment that can be anywhere.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    /// The username of the author of this comment. If you want to get the User you should use the `author()` method.
    pub author: String,
    /// The Fullname of the Link this Comment belongs to.
    pub link_id: Fullname,
    /// The fullname of the Subreddit this Comment belongs to.
    pub subreddit_id: Fullname,
    /// The Fullname of the parent (either another Comment, or the Link)
    pub parent_id: Fullname,
    /// The Fullname of this Comment.
    pub name: Fullname,

    pub(crate) replies: Option<Box<Thing>>,
}

impl Comment {
    /// Returns the User that made this comment.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }

    /// Returns the replies to this comment only.
    pub async fn replies<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
    ) -> Result<Vec<Comment>, Error> {
        if let Some(r) = self.replies.clone() {
            return Thing::try_into(*r);
        }
        client.replies(self.link_id.name(), self.name.name()).await
    }

    /// Returns the parent of this comment, if it exists.
    pub async fn parent<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
    ) -> Result<Option<Comment>, Error> {
        if self.parent_id == self.subreddit_id {
            return Ok(None);
        }
        let parent_comment = client.comment(&self.parent_id.name()).await?;
        Ok(Some(parent_comment))
    }

    /// Returns the link where this comment was made.
    pub async fn link<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<Link, Error> {
        client.link(self.link_id.as_ref()).await
    }

    /// Send a reply to this comment.
    pub async fn reply<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
        body: &str,
    ) -> Result<(), Error> {
        client.submit_comment(self.name.as_ref(), body).await
    }
}

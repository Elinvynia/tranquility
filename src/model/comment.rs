//! Contains the comment model.

use crate::{
    auth::Auth,
    client::Client,
    error::Error,
    model::{listing::Listing, user::User},
};
use serde::{Deserialize, Serialize};

/// A comment that can be anywhere.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    /// The total amount of awards that this comment has received.
    pub total_awards_received: u64,
    /// If this comment was approved, when it happened.
    pub approved_at_utc: Option<u64>,
    /// The total amount of upvotes this comment has received.
    pub ups: i64,
    /// The ID of the comment.
    pub id: String,
    /// Replies to this comment.
    pub replies: Listing,
    /// The username of the author of this comment. If you want to get the User you should use the `author()` method.
    pub author: String,
}

impl Comment {
    /// Returns the User that made this comment.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }
}

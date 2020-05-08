//! A post on reddit.

use crate::{auth::Auth, client::Client, error::Error, model::user::User};
use serde::{Deserialize, Serialize};

/// The struct representing a post on reddit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The title of the link.
    pub title: String,
    /// The username of the author who submitted the link.
    pub author: String,
    /// Whether contest mode is enabled for this link.
    pub contest_mode: bool,
}

impl Link {
    /// Retrieves the User struct of the author.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }
}

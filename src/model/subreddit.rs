//! Contains the subreddit struct.

use crate::{auth::Auth, client::Client, error::Error, model::link::Link};
use serde::{Deserialize, Serialize};

/// The struct representing a subreddit.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subreddit {
    description: String,
    display_name: String,
}

impl Subreddit {
    /// Returns the hot posts in a given subreddit.
    pub async fn hot<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<Vec<Link>, Error> {
        client.hot(&self.display_name).await
    }
}

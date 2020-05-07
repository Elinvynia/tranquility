//! Contains the user's subreddit model.

use serde::Deserialize;

/// A subreddit that is part of the user's profile.
#[derive(Deserialize, Debug, Clone)]
pub struct UserSubreddit {
    /// The description of the subreddit.
    pub description: String,
}

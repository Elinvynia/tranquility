//! Contains the user's subreddit model.

use serde::{Deserialize, Serialize};

/// A subreddit that is part of the user's profile.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSubreddit {
    /// The description of the subreddit.
    pub description: String,
}

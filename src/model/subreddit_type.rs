//! The type of a subreddit

use serde::{Deserialize, Serialize};

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a subreddit.
pub enum SubredditType {
    /// Anyone can post to this subreddit.
    Public,
    /// Only certain users can post to this subreddit.
    Private,
}

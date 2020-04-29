//! Contains the subreddit struct.

use serde::{Deserialize, Serialize};

/// The struct representing a subreddit.
#[derive(Serialize, Deserialize, Debug)]
pub struct Subreddit {
    description: String,
}

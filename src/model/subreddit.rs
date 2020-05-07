//! Contains the subreddit struct.

use crate::model::listing::Listing;
use serde::{Deserialize, Serialize};

/// The struct representing a subreddit.
#[derive(Serialize, Deserialize, Debug)]
pub struct Subreddit {
    description: String,
}

impl Subreddit {
    /// Returns the hot posts in a given subreddit.
    pub fn hot() -> Listing {
        todo!()
    }
}

//! A post on reddit.

use serde::Deserialize;

/// The struct representing a post on reddit.
#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    /// The title of the link.
    pub title: String,
}

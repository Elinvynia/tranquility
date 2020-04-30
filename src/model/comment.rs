//! Contains the comment model.

use serde::{Deserialize, Serialize};

/// A comment that can be anywhere.
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    /// The total amount of awards that this comment has received.
    pub total_awards_received: u64,
    /// If this comment was approved, when it happened.
    pub approved_at_utc: Option<u64>,
    /// The total amount of upvotes this comment has received.
    pub ups: i64,
    /// The ID of the comment.
    pub id: String,
}

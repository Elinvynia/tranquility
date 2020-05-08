//! The possible comment sorts.

use serde::{Deserialize, Serialize};

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a submission.
pub enum CommentSort {
    /// Absolute (total karma) ranking.
    Top,
    /// Relative (percentage-based) ranking.
    Best,
    /// Prioritize controversial comments.
    Controversial,
    /// Newest comments.
    New,
}

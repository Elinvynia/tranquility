//! The type of a submission

use serde::{Deserialize, Serialize};

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a submission.
pub enum SubmissionType {
    /// All submissions allowed.
    Any,
    /// Only link submissions allowed.
    Link,
    /// Only text posts allowed.
    Text,
}

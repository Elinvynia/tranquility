//! A way to sort links.

use serde::{Deserialize, Serialize};

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A way to sort links..
pub enum LinkSort {
    /// Posts made in the past hour.
    Hour,
    /// Posts made in the past day.
    Day,
    /// Posts made in the past week.
    Week,
    /// Posts made in the past month
    Month,
    /// Posts made in the past year.
    Year,
    /// All posts.
    All,
}

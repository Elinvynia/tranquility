//! The size for some CSS content

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The width and height.
pub struct Size (u64, u64);

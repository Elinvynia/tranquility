//! Module containing the fullname struct.

use serde::{Deserialize, Serialize};

/// Fullname is the reddit unique ID for a thing, including the type prefix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fullname(String);

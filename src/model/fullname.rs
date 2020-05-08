//! Module containing the fullname struct.

use serde::Deserialize;

/// Fullname is the reddit unique ID for a thing, including the type prefix.
#[derive(Debug, Deserialize, Clone)]
pub struct Fullname(String);

//! Module containing the Listing struct.

use crate::model::{fullname::Fullname, thing::Thing};
use serde::{Deserialize, Serialize};

/// Listing is the general struct returned by most methods, containing the actual data and options to filter the data returned.
#[serde(default)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    /// Legacy way to authenticate mod actions.
    pub modhash: Option<String>,
    /// The amount of Things returned in the children Vec.
    pub dist: Option<u64>,
    /// The actual things.
    pub children: Option<Vec<Thing>>,
    /// The fullname after this listing.
    pub after: Option<Fullname>,
    /// The fullname before this listing.
    pub before: Option<Fullname>,
    /// The maximum number of items to return in this slice of the listing.
    pub limit: Option<u64>,
    /// The number of items already seen in this listing.
    pub count: Option<u64>,
    /// Optional parameter to override what is shown.
    pub show: Option<String>,
}

impl Default for Listing {
    fn default() -> Self {
        Listing {
            modhash: None,
            dist: None,
            children: None,
            after: None,
            before: None,
            limit: None,
            count: None,
            show: None,
        }
    }
}

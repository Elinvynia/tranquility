//! Module containing the Listing struct.

use crate::error::Error;
use crate::model::{comment::Comment, link::Link, misc::Fullname, thing::Thing};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

/// Listing is the general struct returned by most methods, containing the actual data and options to filter the data returned.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    /// Legacy way to authenticate mod actions.
    pub modhash: Option<String>,
    /// The amount of Things returned in the children Vec.
    pub dist: Option<u64>,
    /// The actual things.
    pub children: Vec<Thing>,
    /// The fullname after this listing.
    pub after: Option<Fullname>,
    /// The fullname before this listing.
    pub before: Option<Fullname>,
    /// The maximum number of items to return in this slice of the listing.
    #[serde(default)]
    pub limit: Option<u64>,
    /// The number of items already seen in this listing.
    #[serde(default)]
    pub count: Option<u64>,
    /// Optional parameter to override what is shown.
    #[serde(default)]
    pub show: Option<String>,
}

impl Default for Listing {
    fn default() -> Self {
        Listing {
            modhash: None,
            dist: None,
            children: Vec::new(),
            after: None,
            before: None,
            limit: None,
            count: None,
            show: None,
        }
    }
}

impl TryFrom<Listing> for Vec<Comment> {
    type Error = Error;
    fn try_from(value: Listing) -> Result<Self, Self::Error> {
        let comments: Result<Vec<Comment>, Error> =
            value.children.into_iter().map(Thing::try_into).collect();
        comments
    }
}

impl TryFrom<Listing> for Vec<Link> {
    type Error = Error;
    fn try_from(value: Listing) -> Result<Self, Self::Error> {
        let comments: Result<Vec<Link>, Error> =
            value.children.into_iter().map(Thing::try_into).collect();
        comments
    }
}

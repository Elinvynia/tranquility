use crate::model::{award::Award, fullname::Fullname, thing::Thing};
use serde::Deserialize;

#[serde(default)]
#[derive(Debug, Deserialize)]
pub struct Listing {
    after: Option<Fullname>,
    before: Option<Fullname>,
    data: Thing,
    limit: Option<u64>,
    count: Option<u64>,
    show: Option<String>,
}

impl Default for Listing {
    fn default() -> Self {
        Listing {
            after: None,
            before: None,
            data: Thing::Award(Award {}),
            limit: None,
            count: None,
            show: None,
        }
    }
}

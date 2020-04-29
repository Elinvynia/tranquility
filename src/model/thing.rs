//! A model for the reddit response wrapper.

use serde_json::{Map, Value};
use crate::error::Error;

/// An enum representing the kind of wrapped reddit API responses.
#[derive(Debug)]
pub enum Thing {
    /// Comment "t1"
    Comment,
    /// Account "t2"
    Account,
    /// Link "t3"
    Link,
    /// Message "t4"
    Message,
    /// Subreddit "t5"
    Subreddit,
    /// Award "t6"
    Award
}

impl From<&str> for Thing {
    fn from(s: &str) -> Self {
        match s {
            "t1" => Thing::Comment,
            "t2" => Thing::Account,
            "t3" => Thing::Link,
            "t4" => Thing::Message,
            "t5" => Thing::Subreddit,
            "t6" => Thing::Award,
            _ => unreachable!()
        }
    }
}

/// Unwraps the inner data field from an API response.
pub fn unwrap_thing(thing: &str) -> Result<String, Error> {
    let x: Map<String, Value> = serde_json::from_str(thing)?;
    let y = x.get("data");
    let s = serde_json::to_string(&y)?;
    Ok(s)
}

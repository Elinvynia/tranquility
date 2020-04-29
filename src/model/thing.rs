use serde_json::{Map, Value};
use reqwest::Response;
use crate::error::Error;

pub(crate) enum Thing {
    Comment,
    Account,
    Link,
    Message,
    Subreddit,
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

pub(crate) async fn unwrap_thing(thing: Response) -> Result<String, Error> {
    let body = thing.text().await?;
    let x: Map<String, Value> = serde_json::from_str(&body)?;
    let y = x.get("data");
    let s = serde_json::to_string(&y)?;
    Ok(s)
}

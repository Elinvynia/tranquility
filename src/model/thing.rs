//! A model for the reddit response wrapper.

use crate::error::Error;
use crate::model::{
    award::Award, comment::Comment, link::Link, listing::Listing, message::Message,
    subreddit::Subreddit, user::User,
};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::convert::TryFrom;

/// An enum representing the kind of wrapped reddit API responses.
#[derive(Debug, Clone, Serialize)]
pub enum Thing {
    /// Comment "t1"
    Comment(Comment),
    /// Account "t2"
    Account(User),
    /// Link "t3"
    Link(Link),
    /// Message "t4"
    Message(Message),
    /// Subreddit "t5"
    Subreddit(Subreddit),
    /// Award "t6"
    Award(Award),
    /// Listing "Listing"
    Listing(Listing),
    /// More comments "more"
    More(String),
}

impl<'de> Deserialize<'de> for Thing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let outer: Map<String, Value> = Map::deserialize(deserializer)?;
        let kind = outer
            .get("kind")
            .ok_or_else(|| DeError::custom("expected kind field"))?
            .as_str()
            .ok_or_else(|| DeError::custom("expected kind field"))?;
        let data = outer
            .get("data")
            .ok_or_else(|| DeError::custom("expected data field"))?;

        Ok(match kind {
            "t1" => {
                let value: Comment = serde_json::from_value(data.clone()).map_err(|_| {
                    DeError::custom("failed to deserialize thing data into comment")
                })?;
                Thing::Comment(value)
            }
            "t2" => {
                let value: User = serde_json::from_value(data.clone())
                    .map_err(|_| DeError::custom("failed to deserialize thing data into user"))?;
                Thing::Account(value)
            }
            "t3" => {
                let value: Link = serde_json::from_value(data.clone())
                    .map_err(|_| DeError::custom("failed to deserialize thing data"))?;
                Thing::Link(value)
            }
            "t4" => {
                let value: Message = serde_json::from_value(data.clone()).map_err(|_| {
                    DeError::custom("failed to deserialize thing data into message")
                })?;
                Thing::Message(value)
            }
            "t5" => {
                let value: Subreddit = serde_json::from_value(data.clone()).map_err(|_| {
                    DeError::custom("failed to deserialize thing data into subreddit")
                })?;
                Thing::Subreddit(value)
            }
            "t6" => {
                let value: Award = serde_json::from_value(data.clone())
                    .map_err(|_| DeError::custom("failed to deserialize thing data into award"))?;
                Thing::Award(value)
            }
            "Listing" => {
                let value: Listing = serde_json::from_value(data.clone()).map_err(|_| {
                    DeError::custom("failed to deserialize thing data into listing")
                })?;
                Thing::Listing(value)
            }
            "more" => todo!(),
            _ => unreachable!(),
        })
    }
}

impl TryFrom<Thing> for User {
    type Error = Error;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        match value {
            Thing::Account(user) => Ok(user),
            _ => Err(Error::Serde(DeError::custom(
                "failed to convert Thing to User",
            ))),
        }
    }
}

impl TryFrom<Thing> for Subreddit {
    type Error = Error;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        match value {
            Thing::Subreddit(sub) => Ok(sub),
            _ => Err(Error::Serde(DeError::custom(
                "failed to convert Thing to Subreddit",
            ))),
        }
    }
}

impl TryFrom<Thing> for Comment {
    type Error = Error;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        match value {
            Thing::Comment(c) => Ok(c),
            _ => Err(Error::Serde(DeError::custom(
                "failed to convert Thing to Comment",
            ))),
        }
    }
}

impl TryFrom<Thing> for Link {
    type Error = Error;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        match value {
            Thing::Link(l) => Ok(l),
            _ => Err(Error::Serde(DeError::custom(
                "failed to convert Thing to Link",
            ))),
        }
    }
}

impl TryFrom<Thing> for Listing {
    type Error = Error;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        match value {
            Thing::Listing(l) => Ok(l),
            _ => Err(Error::Serde(DeError::custom(
                "failed to convert Thing to Listing",
            ))),
        }
    }
}

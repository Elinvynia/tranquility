use crate::model::usersubreddit::UserSubreddit;

use serde::{Deserialize, Deserializer, de::Error as DeError};
use serde_json::Map as JsonMap;

#[derive(Debug)]
pub struct User {
    /// The comment karma of the user.
    pub comment_karma: i64,
    /// When the user was created, local timezone.
    pub created: f64,
    /// When the user was created, normalized to UTC.
    pub created_utc: f64,
    pub has_subscribed: bool,
    /// If the user has a verified email.
    pub has_verified_email: bool,
    /// If the user should be hidden from robots.
    pub hide_from_robots: bool,
    /// Link to the image of the user's icon.
    pub icon_img: String,
    pub id: String,
    /// If the user is an employee of Reddit.
    pub is_employee: bool,
    /// If the user is added as a friend.
    pub is_friend: bool,
    /// If the user currently has gold.
    pub is_gold: bool,
    /// If the user is a global moderator.
    pub is_mod: bool,
    pub kind: String,
    /// The link karma of the user.
    pub link_karma: i64,
    /// The name of the user.
    pub name: String,
    /// If the user's snoovatar should be shown.
    pub pref_show_snoovatar: bool,
    /// The user's subreddit.
    pub subreddit: UserSubreddit,
    /// If the user is verified.
    pub verified: bool,
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut outer_map = JsonMap::deserialize(deserializer)?;

        let kind = outer_map.remove("kind")
            .ok_or_else(|| {DeError::custom("expected kind")})
            .and_then(String::deserialize)
            .map_err(DeError::custom)?;

        let map = outer_map.get_mut("data")
           .ok_or_else(|| DeError::custom("expected data field"))?
           .as_object_mut()
           .ok_or_else(|| DeError::custom("expected data to be an object"))?;

        let comment_karma = map.remove("comment_karma")
            .ok_or_else(|| DeError::custom("expected comment_karma"))
            .and_then(i64::deserialize)
            .map_err(DeError::custom)?;

        let created = map.remove("created")
            .ok_or_else(|| DeError::custom("expected created"))
            .and_then(f64::deserialize)
            .map_err(DeError::custom)?;

        let created_utc = map.remove("created_utc")
            .ok_or_else(|| DeError::custom("expected created_utc"))
            .and_then(f64::deserialize)
            .map_err(DeError::custom)?;

        let has_subscribed = map.remove("has_subscribed")
            .ok_or_else(|| DeError::custom("expected has_subscribed"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let has_verified_email = map.remove("has_verified_email")
            .ok_or_else(|| DeError::custom("expected has_verified_email"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let hide_from_robots = map.remove("hide_from_robots")
            .ok_or_else(|| DeError::custom("expected hide_from_robots"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let icon_img = map.remove("icon_img")
            .ok_or_else(|| DeError::custom("expected icon_img"))
            .and_then(String::deserialize)
            .map_err(DeError::custom)?;

        let id = map.remove("id")
            .ok_or_else(|| DeError::custom("expected id"))
            .and_then(String::deserialize)
            .map_err(DeError::custom)?;

        let is_employee = map.remove("is_employee")
            .ok_or_else(|| DeError::custom("expected is_employee"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let is_friend = map.remove("is_friend")
            .ok_or_else(|| DeError::custom("expected is_friend"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let is_gold = map.remove("is_gold")
            .ok_or_else(|| DeError::custom("expected is_gold"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let is_mod = map.remove("is_mod")
            .ok_or_else(|| DeError::custom("expected is_mod"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let link_karma = map.remove("link_karma")
            .ok_or_else(|| DeError::custom("expected link_karma"))
            .and_then(i64::deserialize)
            .map_err(DeError::custom)?;

        let name = map.remove("name")
            .ok_or_else(|| DeError::custom("expected name"))
            .and_then(String::deserialize)
            .map_err(DeError::custom)?;

        let pref_show_snoovatar = map.remove("pref_show_snoovatar")
            .ok_or_else(|| DeError::custom("expected pref_show_snoovatar"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        let subreddit = map.remove("subreddit")
            .ok_or_else(|| DeError::custom("expected subreddit"))
            .and_then(UserSubreddit::deserialize)
            .map_err(DeError::custom)?;

        let verified = map.remove("verified")
            .ok_or_else(|| DeError::custom("expected verified"))
            .and_then(bool::deserialize)
            .map_err(DeError::custom)?;

        Ok(Self {
            comment_karma,
            created,
            created_utc,
            has_subscribed,
            has_verified_email,
            hide_from_robots,
            icon_img,
            id,
            is_employee,
            is_friend,
            is_gold,
            is_mod,
            kind,
            link_karma,
            name,
            pref_show_snoovatar,
            subreddit,
            verified,
        })
    }
}

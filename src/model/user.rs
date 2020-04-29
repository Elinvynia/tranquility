//! Module related to the user struct.

use crate::model::usersubreddit::UserSubreddit;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
/// The struct representing a reddit user.
pub struct User {
    /// The comment karma of the user.
    pub comment_karma: i64,
    /// When the user was created, local timezone.
    pub created: f64,
    /// When the user was created, normalized to UTC.
    pub created_utc: f64,
    /// If the user has a verified email.
    pub has_verified_email: bool,
    /// If the user should be hidden from robots.
    pub hide_from_robots: bool,
    /// Link to the image of the user's icon.
    pub icon_img: String,
    /// The API ID of the user.
    pub id: String,
    /// If the user is an employee of Reddit.
    pub is_employee: bool,
    /// If the user is added as a friend.
    pub is_friend: bool,
    /// If the user currently has gold.
    pub is_gold: bool,
    /// If the user is a global moderator.
    pub is_mod: bool,
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

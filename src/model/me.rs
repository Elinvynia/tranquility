use crate::model::subreddit::Subreddit;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    pub subreddit: Subreddit,
    pub num_friends: u32,
    pub comment_karma: u32,
    pub link_karma: u32,
    pub verified: bool,
    pub id: String,
    pub created: f32,
    pub created_utc: f32,
}

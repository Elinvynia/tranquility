//! A post on reddit.

use crate::{
    auth::Auth,
    client::Client,
    error::Error,
    model::{fullname::Fullname, subreddit::Subreddit, subreddit_type::SubredditType, user::User},
};
use serde::{Deserialize, Serialize};

/// The struct representing a post on reddit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The title of the link.
    pub title: String,
    /// The username of the author who submitted the link.
    pub author: String,
    /// Whether contest mode is enabled for this link.
    pub contest_mode: bool,
    /// The amount of upvotes.
    pub ups: u64,
    /// The amount of downvotes.
    pub downs: u64,
    /// The `Fullname` of the user that submitted this post.
    pub author_fullname: Fullname,
    /// The score of this post.
    pub score: i64,
    /// If this post has been edited.
    pub edited: bool,
    /// If this post is archived (can't vote or comment on it anymore)
    pub archived: bool,
    /// Whether this post is currently pinned.
    pub pinned: bool,
    /// The amount of comments.
    pub num_comments: u64,
    /// If robots can index this post.
    pub is_robot_indexable: bool,
    /// If the score is hidden or not.
    pub hide_score: bool,
    /// The ratio of upvotes to downvotes.
    pub upvote_ratio: f64,
    /// The `Fullname` of this link.
    #[serde(rename = "name")]
    pub fullname: Fullname,
    /// The type of the subreddit where this link was posted.
    pub subreddit_type: SubredditType,
    /// The name of the subreddit where this link was posted.
    pub subreddit: String,
    /// The subreddit name prefixed with `r/`
    pub subreddit_name_prefixed: String,
    /// If this post is hidden.
    pub hidden: bool,
}

impl Link {
    /// Retrieves the User struct of the author.
    pub async fn author<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<User, Error> {
        client.user(&self.author).await
    }

    /// Retrieves the Subreddit struct where this Link was posted.
    pub async fn subreddit<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
    ) -> Result<Subreddit, Error> {
        client.subreddit(&self.subreddit).await
    }
}

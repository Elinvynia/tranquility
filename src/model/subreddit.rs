//! Contains the subreddit struct.

use crate::{
    auth::Auth,
    client::route::Route,
    client::Client,
    error::Error,
    model::{
        link::Link,
        misc::{CommentSort, Fullname, QuarantinePermissions, SubredditSubmissionType, SubredditType, Params},
    },
};
use serde::{Deserialize, Serialize};

/// The struct representing a subreddit.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subreddit {
    /// The description of the subreddit.
    pub description: String,
    /// Description including HTML tags.
    pub description_html: String,
    /// A short description exposed to search engines.
    pub public_description: String,
    /// The display name of the subreddit.
    pub display_name: String,
    /// The display name prefixed with `r/`
    pub display_name_prefixed: String,
    /// Amount of users subscribed.
    pub subscribers: u64,
    /// Title displayed in the browser tab.
    pub title: String,
    /// If the logged in user is a moderator.
    pub user_is_moderator: bool,
    /// The ID of the subreddit.
    pub id: String,
    /// The fullname of the subreddit.
    pub name: Fullname,
    /// The type of the subreddit.
    pub subreddit_type: SubredditType,
    /// If the subreddit is quarantined.
    /// NOTE: You must opt-in on a subreddit basis to be able to access them via the API.
    /// To opt-in, simply visit the subreddit in your browser while logged in as the user that will be using the bot.
    pub quarantine: bool,
    /// If the subreddit is quarantined, the permissions it has.
    pub quarantine_permissions: Option<QuarantinePermissions>,
    /// The amount of minutes the comment score will be hidden for.
    pub comment_score_hide_mins: u64,
    /// Whether spoilers are enabled.
    pub spoilers_enabled: bool,
    /// The type of submissions that is allowed.
    pub submission_type: SubredditSubmissionType,
    /// If the current user is a subscriber.
    pub user_is_subscriber: bool,
    /// If the subreddit is NSFW
    pub over18: bool,
    /// Whether traffic data is public.
    pub public_traffic: bool,
    /// If the OC tag is enabled.
    pub original_content_tag_enabled: bool,
    /// Whether commenting is restricted.
    pub restrict_commenting: bool,
    /// Optional comment sort for posts in this subreddit
    pub suggested_comment_sort: Option<CommentSort>,
    /// If this subreddit is discoverable via searching.
    pub allow_discovery: bool,
}

impl Subreddit {
    /// Returns the hot posts in a given subreddit.
    pub async fn hot<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<Vec<Link>, Error> {
        client
            .get_posts(Route::SubredditHot(self.display_name.clone()))
            .await
    }

    /// Returns the new posts in a given subreddit.
    #[allow(clippy::new_ret_no_self)]
    pub async fn new<T: Auth + Send + Sync>(&self, client: &Client<T>) -> Result<Vec<Link>, Error> {
        client
            .get_posts(Route::SubredditNew(self.display_name.clone()))
            .await
    }

    /// Returns the rising posts in a given subreddit.
    pub async fn rising<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
    ) -> Result<Vec<Link>, Error> {
        client
            .get_posts(Route::SubredditRising(self.display_name.clone()))
            .await
    }

    /// Post to a subreddit.
    pub async fn submit<'a, T: Auth + Send + Sync>(&self, client: &'a Client<T>) -> SubmitBuilder<'a, T> {
        SubmitBuilder::new(client)
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SubmitBuilder<'a, T: Auth + Send + Sync>{
    pub params: Params,
    pub client: &'a Client<T>,
}

impl<'a, T: Auth + Send + Sync> SubmitBuilder<'a, T> {
    pub fn new(client: &'a Client<T>) -> Self {
        SubmitBuilder { params: Params::new().add("api_type", "json").add("sr", ""), client }
    }

    pub fn spoiler(mut self) -> Self {
        self.params = self.params.add("spoiler", "true");
        self
    }

    /// Whether to send replies
    pub fn send_replies(mut self) -> Self {
        self.params = self.params.add("sendreplies", "true");
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.params = self.params.add("title", title);
        self
    }

    pub async fn send(&self) -> Result<(), Error> {
        self.client.post(Route::Submit, &self.params).await.and(Ok(()))
    }
}

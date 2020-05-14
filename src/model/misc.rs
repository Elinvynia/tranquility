//! Various enums that you can match on.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The quarantine permissions.
pub struct QuarantinePermissions {
    /// If styles are allowed.
    pub styles: bool,
    /// If sharing is allowed.
    pub sharing: bool,
    /// If subreddit images are allowed.
    pub sr_images: bool,
    /// If the subscriber count is visible.
    pub subscriber_count: bool,
    /// If media is allowed.
    pub media: bool,
    /// If polls are allowed.
    pub polls: bool,
    /// If videos are allowed.
    pub videos: bool,
    /// If images are allowed.
    pub images: bool,
    /// If crossposts are allowed.
    pub crossposts: bool,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a subreddit.
pub enum SubredditType {
    /// Anyone can post to this subreddit.
    Public,
    /// Only certain users can post to this subreddit.
    Private,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a submission.
pub enum CommentSort {
    /// Absolute (total karma) ranking.
    Top,
    /// Relative (percentage-based) ranking.
    Best,
    /// Prioritize controversial comments.
    Controversial,
    /// Newest comments.
    New,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// The type of a submission.
pub enum SubmissionType {
    /// All submissions allowed.
    Any,
    /// Only link submissions allowed.
    Link,
    /// Only text posts allowed.
    Text,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A way to sort links..
pub enum LinkSort {
    /// Posts made in the past hour.
    Hour,
    /// Posts made in the past day.
    Day,
    /// Posts made in the past week.
    Week,
    /// Posts made in the past month
    Month,
    /// Posts made in the past year.
    Year,
    /// All posts.
    All,
}

/// Parameters for a GET query, a key-value tuple of Strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params(Vec<(String, String)>);

impl Params {
    /// Creates a new Params struct.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds a parameter.
    pub fn add(mut self, key: &str, value: &str) -> Self {
        self.0.push((key.into(), value.into()));
        self
    }
}

impl Default for Params {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// Fullname is the reddit unique ID for a thing, including the type prefix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fullname(String);

impl AsRef<str> for Fullname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Fullname {
    /// Gets the ID36 portion of a Fullname.
    pub fn name(&self) -> String {
        let parts: Vec<&str> = self.as_ref().split('_').collect();
        parts[1].to_owned()
    }

    /// Gets the prefix of a Fullname (t1, t2, ...)
    pub fn prefix(&self) -> String {
        let parts: Vec<&str> = self.as_ref().split('_').collect();
        parts[0].to_owned()
    }
}

//! The quarantine permissions

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

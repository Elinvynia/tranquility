//! The module for the award struct.

use crate::{
    auth::Auth,
    client::{route::Route, Client},
    error::Error,
    model::{misc::AwardSubtype, misc::AwardType, misc::Params},
};
use serde::{Deserialize, Serialize};

/// The struct representing an award.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Award {
    /// The ID of the award.
    pub id: String,
    /// If not global, which subreddit it belongs to.
    pub subreddit_id: Option<String>,
    /// How expensive the award is.
    pub coin_price: u64,
    /// The URL for the icon.
    pub icon_url: String,
    /// How many days of premium it gives.
    pub days_of_premium: u64,
    /// The height of the icon.
    pub icon_height: u64,
    /// The width of the icon.
    pub icon_width: u64,
    /// Whether the award is currently enabled.
    pub is_enabled: bool,
    /// The description of the award.
    pub description: String,
    /// How many times was this reward used on a Thing.
    pub count: u64,
    /// The name of the award.
    pub name: String,
    /// The subtype of the award.
    pub award_sub_type: AwardSubtype,
    /// The type of the award.
    pub award_type: AwardType,
}

impl Award {
    /// Brings the award to the attention of the admins.
    pub async fn report<T: Auth + Send + Sync>(
        &self,
        client: &Client<T>,
        reason: &str,
    ) -> Result<(), Error> {
        client
            .post(
                Route::ReportAward,
                &Params::new()
                    .add("award_id", self.id.as_ref())
                    .add("reason", reason),
            )
            .await
            .and(Ok(()))
    }
}

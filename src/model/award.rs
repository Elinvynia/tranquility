//! The module for the award struct.

use crate::{
    auth::Auth,
    client::{route::Route, Client},
    error::Error,
    model::{misc::Fullname, misc::Params},
};
use serde::{Deserialize, Serialize};

/// The struct representing an award.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Award {
    /// The Fullname of the award.
    pub id: Fullname,
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

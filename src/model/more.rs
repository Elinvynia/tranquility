use crate::model::misc::Fullname;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct More {
    pub count: u64,
    pub name: Fullname,
    pub id: String,
    pub parent_id: Fullname,
    pub depth: u64,
    pub children: Vec<String>,
    pub after: Option<String>,
    pub before: Option<String>,
}

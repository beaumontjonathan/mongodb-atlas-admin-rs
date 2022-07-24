use chrono::{DateTime, Utc};
use serde::Deserialize;

mod create_one;
mod delete_one;
mod get_all;

pub use create_one::{CreateOneProject, CreateOneProjectData};
pub use delete_one::*;
pub use get_all::{GetAllProject, GetAllProjectData};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub cluster_count: usize,
    pub created: DateTime<Utc>,
    pub id: String,
    pub name: String,
    pub org_id: String,
}

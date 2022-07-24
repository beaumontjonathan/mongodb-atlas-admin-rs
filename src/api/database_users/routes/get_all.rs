use std::borrow::Cow;

use http::Method;
use reqwest::Response;
use serde::Deserialize;

use super::super::DatabaseUser;
use crate::Endpoint;

pub struct GetAllDatabaseUsers {
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllDatabaseUsersData {
    pub results: Vec<DatabaseUser>,
    pub total_count: usize,
}

#[derive(Debug, thiserror::Error)]
#[error("Error")]
pub struct Error;

#[async_trait::async_trait]
impl Endpoint for GetAllDatabaseUsers {
    type Data = GetAllDatabaseUsersData;
    type Error = Error;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("/groups/{}/databaseUsers", self.group_id).into()
    }

    async fn get_data(res: Response) -> Result<Self::Data, Error> {
        Ok(res.json::<Self::Data>().await.unwrap())
    }
}

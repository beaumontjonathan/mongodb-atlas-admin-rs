use std::borrow::Cow;

use http::Method;
use reqwest::Response;
use serde::Deserialize;

use super::Project;
use crate::Endpoint;

pub struct GetAllProject;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllProjectData {
    pub results: Vec<Project>,
    pub total_count: usize,
}

#[derive(Debug, thiserror::Error)]
#[error("Error")]
pub struct Error;

#[async_trait::async_trait]
impl Endpoint for GetAllProject {
    type Data = GetAllProjectData;
    type Error = Error;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        Cow::from("/groups")
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        Ok(res.json::<Self::Data>().await.unwrap())
    }
}

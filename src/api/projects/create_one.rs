use std::borrow::Cow;

use http::Method;
use reqwest::Response;
use serde::Serialize;

use super::Project;
use crate::Endpoint;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOneProject {
    pub name: String,
    pub org_id: String,
}

pub type CreateOneProjectData = Project;

#[derive(Debug, thiserror::Error)]
#[error("Error")]
pub struct Error;

#[async_trait::async_trait]
impl Endpoint for CreateOneProject {
    type Data = CreateOneProjectData;
    type Error = Error;

    fn method() -> Method {
        Method::POST
    }

    fn path(&self) -> Cow<'static, str> {
        Cow::from("/groups")
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(Some(serde_json::to_vec(self).unwrap()))
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        Ok(res.json::<Self::Data>().await.unwrap())
    }
}

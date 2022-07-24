use std::borrow::Cow;

use http::{Method, StatusCode};
use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::{endpoint::RequestError, Endpoint};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOneProject {
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteOneProjectData {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad request JSON")]
    BadRequestJson(#[from] serde_json::Error),
    #[error("Bad request")]
    BadRequest(RequestError),
    #[error("Conflict")]
    Conflict(RequestError),
    #[error("Internal server error")]
    InternalServerError(RequestError),
    #[error("Bad response")]
    BadResponse(#[from] reqwest::Error),
}

#[async_trait::async_trait]
impl Endpoint for DeleteOneProject {
    type Data = DeleteOneProjectData;
    type Error = Error;

    fn method() -> Method {
        Method::DELETE
    }

    fn path(&self) -> Cow<'static, str> {
        format!("/groups/{}", self.group_id).into()
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(Some(serde_json::to_vec(self)?))
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        match res.status() {
            StatusCode::NO_CONTENT => Ok(res.json::<Self::Data>().await?),
            StatusCode::BAD_REQUEST => Err(Error::BadRequest(res.json::<RequestError>().await?)),
            StatusCode::CONFLICT => Err(Error::Conflict(res.json::<RequestError>().await?)),
            _ => todo!(),
        }
    }
}

use http::{Method, StatusCode};
use reqwest::Response;
use serde::Deserialize;
use std::borrow::Cow;

use crate::{
    access_tracking::{AccessHistoryOptions, AccessLog},
    Endpoint, RequestError, ThisError,
};

/// https://www.mongodb.com/docs/atlas/reference/api-resources-spec/#operation/returnDatabaseAccessHistoryForOneClusterUsingItsClusterName
#[derive(Debug)]
pub struct GetOneClusterAccessHistoryByClusterName {
    /// Unique 24-hexadecimal digit string that identifies your project.
    pub group_id: String,
    /// Human-readable label that identifies the cluster.
    pub cluster_name: String,
    pub options: AccessHistoryOptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOneClusterAccessHistoryByClusterNameData {
    pub access_logs: Vec<AccessLog>,
}

#[derive(Debug, ThisError)]
pub enum GetOneClusterAccessHistoryByClusterNameError {
    #[error("Bad request JSON")]
    BadRequestJson(#[from] serde_json::Error),
    #[error("Bad request")]
    BadRequest(RequestError),
    #[error("Unauthorized")]
    Unauthorized(RequestError),
    #[error("Internal server error")]
    InternalServerError(RequestError),
    #[error("Bad response")]
    BadResponse(#[from] reqwest::Error),
    #[error("Unknown response {0}")]
    UnknownResponse(StatusCode),
}

#[async_trait::async_trait]
impl Endpoint for GetOneClusterAccessHistoryByClusterName {
    type Data = GetOneClusterAccessHistoryByClusterNameData;
    type Error = GetOneClusterAccessHistoryByClusterNameError;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!(
            "/groups/{}/dbAccessHistory/clusters/{}",
            self.group_id, self.cluster_name
        )
        .into()
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(Some(serde_json::to_vec(&self.options)?))
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        match res.status() {
            StatusCode::OK => Ok(res.json::<Self::Data>().await?),
            StatusCode::BAD_REQUEST => {
                Err(GetOneClusterAccessHistoryByClusterNameError::BadRequest(
                    res.json::<RequestError>().await?,
                ))
            }
            StatusCode::UNAUTHORIZED => {
                Err(GetOneClusterAccessHistoryByClusterNameError::Unauthorized(
                    res.json::<RequestError>().await?,
                ))
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(
                GetOneClusterAccessHistoryByClusterNameError::InternalServerError(
                    res.json::<RequestError>().await?,
                ),
            ),
            code => Err(GetOneClusterAccessHistoryByClusterNameError::UnknownResponse(code)),
        }
    }
}

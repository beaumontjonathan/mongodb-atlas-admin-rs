use http::{Method, StatusCode};
use reqwest::Response;
use std::borrow::Cow;

use crate::{
    access_tracking::{AccessHistoryOptions, AccessLog},
    Endpoint, RequestError, ThisError,
};

/// https://www.mongodb.com/docs/atlas/reference/api-resources-spec/#operation/returnDatabaseAccessHistoryForOneClusterUsingItsHostname
#[derive(Debug)]
pub struct GetOneClusterAccessHistoryByHostname {
    /// Unique 24-hexadecimal digit string that identifies your project.
    pub group_id: String,
    /// Fully qualified domain name or IP address of the MongoDB host that stores the log files that you want to download.
    pub hostname: String,
    pub options: AccessHistoryOptions,
}

pub type GetOneClusterAccessHistoryByHostnameData = Vec<AccessLog>;

#[derive(Debug, ThisError)]
pub enum GetOneClusterAccessHistoryByHostNameError {
    #[error("Bad request JSON")]
    BadRequestJson(#[from] serde_json::Error),
    #[error("Not found")]
    NotFound(RequestError),
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
impl Endpoint for GetOneClusterAccessHistoryByHostname {
    type Data = GetOneClusterAccessHistoryByHostnameData;
    type Error = GetOneClusterAccessHistoryByHostNameError;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!(
            "/groups/{}/dbAccessHistory/processes/{}",
            self.group_id, self.hostname
        )
        .into()
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(Some(serde_json::to_vec(&self.options)?))
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        match res.status() {
            StatusCode::OK => Ok(res.json::<Self::Data>().await?),
            StatusCode::UNAUTHORIZED => {
                Err(GetOneClusterAccessHistoryByHostNameError::Unauthorized(
                    res.json::<RequestError>().await?,
                ))
            }
            StatusCode::NOT_FOUND => Err(GetOneClusterAccessHistoryByHostNameError::NotFound(
                res.json::<RequestError>().await?,
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(
                GetOneClusterAccessHistoryByHostNameError::InternalServerError(
                    res.json::<RequestError>().await?,
                ),
            ),
            code => Err(GetOneClusterAccessHistoryByHostNameError::UnknownResponse(
                code,
            )),
        }
    }
}

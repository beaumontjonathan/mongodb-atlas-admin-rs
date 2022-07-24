use std::borrow::Cow;

use http::{Method, StatusCode};
use reqwest::Response;
use serde::Deserialize;

use crate::{
    alert_configurations::ApiAlertConfigView, Endpoint, Pagination, RequestError, ThisError,
};

/// https://www.mongodb.com/docs/atlas/reference/api-resources-spec/#operation/returnAllAlertConfigurationsForOneProject
#[derive(Debug)]
pub struct GetAllAlertConfigsForProject {
    /// Application that handles this API resource.
    pub app: String,
    /// Unique 24-hexadecimal digit string that identifies your project.
    pub group_id: String,
    pub pagination: Pagination,
}

impl GetAllAlertConfigsForProject {
    pub fn new(group_id: String, pagination: Pagination) -> Self {
        Self {
            app: "atlas".to_owned(),
            group_id,
            pagination,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAlertConfigsForProjectData {
    pub results: Vec<ApiAlertConfigView>,
    pub total_count: isize,
}

#[derive(Debug, ThisError)]
pub enum GetAllAlertConfigsForProjectError {
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
impl Endpoint for GetAllAlertConfigsForProject {
    type Data = GetAllAlertConfigsForProjectData;
    type Error = GetAllAlertConfigsForProjectError;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!(
            "/groups/{}/alertConfigs?{}",
            self.group_id,
            self.pagination.to_query_params()
        )
        .into()
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        println!("res: {:#?}", &res);
        match res.status() {
            StatusCode::OK => Ok(res.json::<Self::Data>().await?),
            StatusCode::UNAUTHORIZED => Err(GetAllAlertConfigsForProjectError::Unauthorized(
                res.json::<RequestError>().await?,
            )),
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(GetAllAlertConfigsForProjectError::InternalServerError(
                    res.json::<RequestError>().await?,
                ))
            }
            code => Err(GetAllAlertConfigsForProjectError::UnknownResponse(code)),
        }
    }
}

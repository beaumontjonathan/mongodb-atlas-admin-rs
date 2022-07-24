use std::borrow::Cow;

use http::{Method, StatusCode};
use reqwest::Response;

use crate::{
    alert_configurations::AlertConfigurationMatchersFieldNames, Endpoint, RequestError, ThisError,
};

/// https://www.mongodb.com/docs/atlas/reference/api-resources-spec/#operation/returnAlertConfigMatchersFieldNames
#[derive(Debug)]
pub struct GetAllAlertConfigMatchersFieldNames {
    /// Application that handles this API resource.
    pub app: String,
}

impl Default for GetAllAlertConfigMatchersFieldNames {
    fn default() -> Self {
        Self {
            app: "atlas".to_owned(),
        }
    }
}

pub type GetAllAlertConfigMatchersFieldNamesData = Vec<AlertConfigurationMatchersFieldNames>;

#[derive(Debug, ThisError)]
pub enum GetAllAlertConfigMatchersFieldNamesError {
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
impl Endpoint for GetAllAlertConfigMatchersFieldNames {
    type Data = GetAllAlertConfigMatchersFieldNamesData;
    type Error = GetAllAlertConfigMatchersFieldNamesError;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "/alertConfigs/matchers/fieldNames".into()
    }

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error> {
        match res.status() {
            StatusCode::OK => Ok(res.json::<Self::Data>().await?),
            StatusCode::UNAUTHORIZED => {
                Err(GetAllAlertConfigMatchersFieldNamesError::Unauthorized(
                    res.json::<RequestError>().await?,
                ))
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(
                GetAllAlertConfigMatchersFieldNamesError::InternalServerError(
                    res.json::<RequestError>().await?,
                ),
            ),
            code => Err(GetAllAlertConfigMatchersFieldNamesError::UnknownResponse(
                code,
            )),
        }
    }
}

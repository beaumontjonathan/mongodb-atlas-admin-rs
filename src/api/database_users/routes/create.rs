use std::borrow::Cow;

use http::Method;
use reqwest::Response;

use super::super::DatabaseUser;
use crate::Endpoint;

pub enum CreateDatabaseUser {
    AwsUserOrRole {
        arn: String,
        group_id: String,
    },
    InternalUser {
        username: String,
        group_id: String,
        database_name: String,
    },
}

impl CreateDatabaseUser {
    fn group_id(&self) -> &str {
        match self {
            Self::AwsUserOrRole { group_id, .. } => group_id,
            Self::InternalUser { group_id, .. } => group_id,
        }
    }

    fn database_name(&self) -> &str {
        match self {
            Self::AwsUserOrRole { .. } => "$external",
            Self::InternalUser {
                database_name: database,
                ..
            } => database,
        }
    }

    fn username(&self) -> Cow<str> {
        match self {
            Self::AwsUserOrRole { arn, .. } => urlencoding::encode(arn),
            Self::InternalUser { username, .. } => urlencoding::encode(username),
        }
    }
}

pub type CreateDatabaseUserData = DatabaseUser;

#[derive(Debug, thiserror::Error)]
#[error("Error")]
pub struct Error;

#[async_trait::async_trait]
impl Endpoint for CreateDatabaseUser {
    type Data = CreateDatabaseUserData;
    type Error = Error;

    fn method() -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!(
            "/groups/{}/databaseUsers/{}/{}",
            self.group_id(),
            self.database_name(),
            self.username()
        )
        .into()
    }

    async fn get_data(res: Response) -> Result<Self::Data, Error> {
        Ok(res.json::<Self::Data>().await.unwrap())
    }
}

use chrono::{DateTime, Utc};

mod routes;
pub use routes::*;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct DatabaseUserLabel {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUserRole {
    pub collection_name: Option<String>,
    pub database_name: Option<String>,
    pub role_name: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DatabaseUserScope {
    pub name: String,
    #[serde(rename = "type")]
    pub scope: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUserAttributes {
    pub delete_after_date: Option<DateTime<Utc>>,
    pub group_id: String,
    pub labels: Vec<DatabaseUserLabel>,
    pub roles: Vec<DatabaseUserRole>,
    pub scopes: Vec<DatabaseUserScope>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "awsIAMType")]
pub enum DatabaseUser {
    #[serde(rename = "USER")]
    AwsIamUser {
        #[serde(rename = "username")]
        arn: String,
        #[serde(flatten)]
        attributes: DatabaseUserAttributes,
    },
    #[serde(rename = "ROLE")]
    AwsIamRole {
        #[serde(rename = "username")]
        arn: String,
        #[serde(flatten)]
        attributes: DatabaseUserAttributes,
    },
    #[serde(rename = "NONE")]
    InternalUser {
        #[serde(rename = "databaseName")]
        database_name: String,
        username: String,
        #[serde(flatten)]
        attributes: DatabaseUserAttributes,
    },
}

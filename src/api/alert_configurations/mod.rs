use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod routes;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertConfigurationMatchersFieldNames {
    TypeName,
    Hostname,
    Port,
    HostnameAndPort,
    ReplicaSetName,
    ShardName,
    ClusterName,
    ApplicationId,
}

/// TODO: None of these types have been _confirmed_, a bunch could probably be
/// enums; this is just the example object provided.
///
/// The same goes for all the child types below.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAlertConfigView {
    pub id: String,
    pub group_id: String,
    pub event_type_name: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub enabled: bool,
    pub type_name: String,
    pub matchers: Vec<ApiAlertConfigViewMatcher>,
    pub metric_threshold: Option<ApiAlertConfigViewMetricThreshold>,
    pub notifications: Vec<ApiAlertConfigViewNotification>,
    pub threshold: Option<ApiAlertConfigViewThreshold>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAlertConfigViewMatcher {
    pub field_name: AlertConfigurationMatchersFieldNames,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAlertConfigViewMetricThreshold {
    pub metric_name: String,
    pub mode: String,
    pub operator: String,
    pub threshold: f64,
    pub units: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAlertConfigViewNotification {
    pub api_token: Option<String>,
    pub channel_name: Option<String>,
    pub datadog_api_key: Option<String>,
    pub datadog_region: Option<String>,
    pub delay_min: usize,
    pub email_address: Option<String>,
    pub email_enabled: bool,
    pub flow_name: Option<String>,
    pub flowdock_api_token: Option<String>,
    pub interval_min: usize,
    pub microsoft_teams_webhook_url: Option<String>,
    pub mobile_number: Option<String>,
    pub notification_token: Option<String>,
    pub ops_genie_api_key: Option<String>,
    pub ops_genie_region: Option<String>,
    pub org_name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub room_name: Option<String>,
    pub service_key: Option<String>,
    pub severity: Option<String>,
    pub ssm_enabled: Option<bool>,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
    pub type_name: String,
    pub username: Option<String>,
    pub victor_ops_api_key: Option<String>,
    pub victor_ops_routing_key: Option<String>,
    pub webhook_secret: Option<String>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAlertConfigViewThreshold {
    pub operator: String,
    pub threshold: usize,
    pub units: String,
}

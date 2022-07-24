use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod routes;

#[derive(Debug, Serialize, Default)]
pub struct AccessHistoryOptions {
    #[serde(rename = "authResult")]
    /// Flag that indicates whether the response returns the successful authentication attempts only.
    pub auth_result: Option<bool>,
    #[serde(rename = "ipAddress")]
    /// One Internet Protocol address that attempted to authenticate with the database.
    pub ip_address: Option<std::net::IpAddr>,
    #[serde(rename = "nLogs")]
    /// Maximum number of lines from the log to return.
    /// Range [0 .. 20000], default 20000
    pub n_logs: Option<i16>,
    #[serde(flatten)]
    pub interval: Option<AccessHistoryOptionsInterval>,
}

#[derive(Debug, Serialize)]
pub struct AccessHistoryOptionsInterval {
    // Date and time when MongoDB Cloud begins retrieving database history.
    pub start: DateTime<Utc>,
    /// Date and time when to stop retrieving database history.
    pub end: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessLog {
    /// Flag that indicates whether the response should return successful authentication attempts only.
    pub auth_result: bool,
    /// Database against which someone attempted to authenticate.
    pub auth_source: String,
    /// Reason that the authentication failed. Null if authentication succeeded.
    pub failure_reason: Option<String>,
    /// Unique 24-hexadecimal character string that identifies the project.
    pub group_id: String,
    /// Human-readable label that identifies the hostname of the target node that received the authentication attempt.
    pub hostname: String,
    /// Internet Protocol address that attempted to authenticate with the database.
    pub ip_address: std::net::IpAddr,
    /// Text of the host log concerning the authentication attempt.
    pub log_line: String,
    /// Date and time when someone made this authentication attempt.
    pub timestamp: DateTime<Utc>,
    /// Username used to authenticate against the database.
    pub username: String,
}

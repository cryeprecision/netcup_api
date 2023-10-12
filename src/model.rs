use crate::{de, ser};

/// Create a login session for API users
#[derive(Debug, Clone, serde::Serialize)]
pub struct Login {
    #[serde(rename = "customernumber")]
    #[serde(serialize_with = "ser::with_to_string")]
    pub(crate) customer_nr: i64,
    #[serde(rename = "apikey")]
    pub(crate) api_key: String,
    #[serde(rename = "apipassword")]
    pub(crate) api_password: String,
    #[serde(rename = "clientrequestid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) client_req_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Auth {
    #[serde(rename = "customernumber")]
    #[serde(serialize_with = "ser::with_to_string")]
    pub(crate) customer_nr: i64,
    #[serde(rename = "apikey")]
    pub(crate) api_key: String,
    #[serde(rename = "apisessionid")]
    pub(crate) api_session_id: String,
    #[serde(rename = "clientrequestid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) client_req_id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Response {
    #[serde(rename = "serverrequestid")]
    pub(crate) server_req_id: String,
    #[serde(rename = "clientrequestid")]
    #[serde(deserialize_with = "de::empty_string_as_none_opt")]
    pub(crate) client_req_id: Option<String>,
    #[serde(rename = "action")]
    #[serde(deserialize_with = "de::empty_string_as_none")]
    pub(crate) action: Option<String>,
    #[serde(rename = "status")]
    pub(crate) status: String,
    #[serde(rename = "statuscode")]
    pub(crate) status_code: i64,
    #[serde(rename = "shortmessage")]
    pub(crate) short_msg: String,
    #[serde(rename = "longmessage")]
    #[serde(deserialize_with = "de::empty_string_as_none_opt")]
    pub(crate) long_msg: Option<String>,
    #[serde(rename = "responsedata")]
    pub(crate) response_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DnsZone {
    #[serde(rename = "domainname")]
    domain_name: String,
    #[serde(rename = "ttl")]
    ttl: i64,
    #[serde(rename = "serial")]
    serial: i64,
    #[serde(rename = "refresh")]
    refresh: i64,
    #[serde(rename = "retry")]
    retry: i64,
    #[serde(rename = "expire")]
    expire: i64,
    #[serde(rename = "dnssecstatus")]
    dnssec_status: bool,
}

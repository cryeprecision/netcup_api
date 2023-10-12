pub struct Login {
    customer_nr: i64,
    api_key: String,
    api_password: String,
    client_req_id: Option<String>,
}

pub struct Auth {
    customer_nr: i64,
    api_key: String,
    api_session_id: String,
    client_req_id: Option<String>,
}

pub struct Response {
    server_req_id: String,
    client_req_id: Option<String>,
    action: String,
    status: String,
    status_code: i64,
    short_msg: String,
    long_msg: Option<String>,
    response_data: Option<String>,
}

pub struct Session {
    api_session_id: String,
}

pub struct DnsZone {
    domain_name: String,
    ttl: i64,
    serial: i64,
    refresh: i64,
    retry: i64,
    expire: i64,
    dnssec_status: bool,
}

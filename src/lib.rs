mod action;
mod client;
mod de;
mod model;
mod ser;

#[cfg(test)]
mod test;

const NETCUP_API: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_env() {
        let env = test::Env::load().unwrap();
        println!("env: {:?}", env);
    }

    #[tokio::test]
    async fn response() {
        let client = test::reqwest_client();

        let resp = client.get(NETCUP_API).send().await;
        let resp = resp.unwrap().bytes().await.unwrap();

        let json: model::Response = serde_json::from_slice(&resp).unwrap();
        println!("{:?}", json);
    }
}

use std::fmt::Debug;

use anyhow::Context;

use crate::{action, model, NETCUP_API};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Payload<T>
where
    T: serde::Serialize + Clone + Debug,
{
    #[serde(rename = "action")]
    pub(crate) action: action::Action,
    #[serde(rename = "param")]
    pub(crate) payload: T,
}

impl<T> Payload<T>
where
    T: serde::Serialize + Clone + Debug,
{
    pub async fn send(&self, client: &reqwest::Client) -> anyhow::Result<model::Response>
    where
        T: serde::Serialize + Clone + Debug,
    {
        let payload = serde_json::to_vec(self).context("couldn't serialize payload to json")?;
        log::debug!(
            "json payload: {}",
            std::str::from_utf8(&payload).unwrap_or("invalid utf-8")
        );

        let resp = client.post(NETCUP_API).body(payload).send().await;
        let resp = resp.context("couldn't post payload to api")?;
        let resp = resp.error_for_status().context("unexpected status code")?;

        let resp = resp
            .bytes()
            .await
            .context("couldn't get bytes of response body")?;

        let json: model::Response =
            serde_json::from_slice(&resp).context("couldn't deserialize response body")?;
        log::debug!("json reponse: {:#?}", json);

        if json.status != "success" {
            let long_msg = json.long_msg.as_ref().map(|s| s.as_str()).unwrap_or("🤷");
            anyhow::bail!("{}: {}", json.short_msg, long_msg);
        }

        Ok(json)
    }
}

/// A client that is logged in
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
    pub(crate) auth: model::Auth,
}

impl Client {
    pub async fn login(
        client: reqwest::Client,
        customer_nr: i64,
        api_key: String,
        api_password: String,
        client_req_id: Option<String>,
    ) -> anyhow::Result<Client> {
        let payload = Payload {
            action: action::Action::Login,
            payload: model::Login {
                customer_nr,
                api_key,
                api_password,
                client_req_id,
            },
        };
        log::debug!("login raw payload: {:#?}", payload);

        let resp = payload.send(&client).await?;
        log::debug!("login raw response: {:#?}", resp);

        // TODO: session id in resp.response_data

        unimplemented!();
    }

    /// Logout from the current session.
    ///
    /// Returns the [`reqwest::Client`] so it can be reused.
    pub async fn logout(self) -> anyhow::Result<reqwest::Client> {
        let payload = Payload {
            action: action::Action::Logout,
            payload: self.auth,
        };
        log::debug!("logout raw payload: {:#?}", payload);

        let resp = payload.send(&self.inner).await?;
        log::debug!("logout raw response: {:#?}", resp);

        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn login() {
        test::init_logger().unwrap();
        let client = test::reqwest_client();
        let env = test::Env::load().unwrap();

        let client = Client::login(client, env.customer_nr, env.api_key, env.api_password, None)
            .await
            .unwrap();

        // bla

        client.logout().await.unwrap();
    }
}

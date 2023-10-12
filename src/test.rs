use anyhow::Context;
use log::LevelFilter;
use simplelog::{format_description, ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

#[derive(Debug, Clone)]
pub(crate) struct Env {
    pub(crate) customer_nr: i64,
    pub(crate) api_key: String,
    pub(crate) api_password: String,
}

impl Env {
    pub(crate) fn load() -> anyhow::Result<Env> {
        fn load(key: &str) -> anyhow::Result<String> {
            dotenv::var(key).with_context(|| format!("couldn't find env variable {}", key))
        }

        let env = Env {
            customer_nr: load("NETCUP_CUSTOMER_NR")?.parse()?,
            api_key: load("NETCUP_API_KEY")?,
            api_password: load("NETCUP_API_PASSWORD")?,
        };
        log::debug!("env loaded: {:?}", env);

        Ok(env)
    }
}

pub(crate) fn reqwest_client() -> reqwest::Client {
    reqwest::Client::builder()
        .https_only(true)
        .min_tls_version(reqwest::tls::Version::TLS_1_2)
        .build()
        .unwrap()
}

pub(crate) fn init_logger() -> anyhow::Result<()> {
    let mut config = ConfigBuilder::default();

    config
        .set_target_level(LevelFilter::Debug)
        .set_location_level(LevelFilter::Off)
        .set_time_level(LevelFilter::Debug)
        .set_time_format_custom(format_description!(
            version = 2,
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory][offset_minute]"
        ));

    config.set_time_offset_to_local().unwrap();

    TermLogger::init(
        LevelFilter::Debug,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .context("couldn't init term logger")
}

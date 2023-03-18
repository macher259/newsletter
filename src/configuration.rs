const APP_ENV_VAR: &str = "APP_ENVIRONMENT";

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

pub enum Environment {
    Local,
    Production,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var(APP_ENV_VAR)
        .unwrap_or_else(|_| "local".to_owned())
        .try_into()
        .unwrap_or_else(|_| panic!("Failed to parse {} variable.", APP_ENV_VAR));
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::with_name(
            configuration_directory.join("base").to_str().unwrap(),
        ))
        .add_source(config::File::with_name(
            configuration_directory
                .join(environment_filename)
                .to_str()
                .unwrap(),
        ))
        .build()?;

    settings.try_deserialize()
}

impl Environment {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

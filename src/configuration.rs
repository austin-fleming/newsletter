use actix_web::cookie::time::Date;

use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: Secret<String>,
    pub port: Secret<u16>,
    pub username: Secret<String>,
    pub password: Secret<String>,
    pub database_name: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: Secret<u16>,
    pub host: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
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
                "{} is not a supported environment. \
            Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username.expose_secret(),
            self.password.expose_secret(),
            self.host.expose_secret(),
            self.port.expose_secret(),
            self.database_name.expose_secret()
        ))
    }

    // Allows connecting to postgres instance without accessing a particular DB
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username.expose_secret(),
            self.password.expose_secret(),
            self.host.expose_secret(),
            self.port.expose_secret()
        ))
    }
}

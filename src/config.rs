use figment::Figment;
use figment::providers::{Serialized, Toml, Env, Format};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub hostname: String,
    pub port: u32,
    pub token: Option<String>,
    pub version: u8,
}

const DEFAULT_CONFIG_FILE : &str = "settings.toml";
const DEFAULT_CONFIG_DIR : &str = ".config/neoctl";
const DEFAULT_CONFIG_HOSTNAME: &str = "localhost";
const DEFAULT_CONFIG_PORT: u32 = 8080;
const DEFAULT_API_VERSION: u8 = 1;

impl Default for Config {
    
    fn default() -> Config {
        Config {
            hostname: DEFAULT_CONFIG_HOSTNAME.to_string(),
            port: DEFAULT_CONFIG_PORT,
            token: None,
            version: DEFAULT_API_VERSION,
        }
    }

}

impl Config {

    pub fn new() -> Self {
        let mut config_path = home::home_dir().expect("Could not found user home directory");
        config_path.push(DEFAULT_CONFIG_DIR);
        config_path.push(DEFAULT_CONFIG_FILE);

        let config: Config = Figment::from(Serialized::defaults(Config::default()))
                                .merge(Toml::file(config_path.as_path()))
                                .merge(Env::prefixed("MATRIX_"))
                                .extract()
                                .expect("fail to initialize configuration");

        if config.token.is_none() {
            panic!("Synapse API token is mandatory. Export MATRIX_TOKEN environment variable.")
        }

        config
    }

}
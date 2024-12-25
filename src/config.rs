use serde::Deserialize;
use std::{
    net::{Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

// Type alias for thread safe app configuration.
pub type Config = Arc<Configuration>;

// App Configuration Struct.
#[derive(Deserialize)]
pub struct Configuration {
    pub env: Environment,
    pub listen_address: SocketAddr,
    pub app_port: u16,
    pub db_str: String,
    pub db_pool_max_size: u32,
    pub rpc_url: String,
}

// Environment application is running in.
#[derive(Deserialize, Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Configuration {
    pub fn new() -> Config {
        // Environment application is running in.
        let env = env_var("APP_ENVIRONMENT")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

        // Port to listen for requests.
        let app_port = env_var("PORT")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

        // DB config parameters.
        let db_str = env_var("DATABASE_URL");
        let db_pool_max_size = env_var("DATABASE_POOL_MAX_SIZE")
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer.");

        // 0.0.0.0 + Port to support containerisation.
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        let rpc_url = env_var("RPC_PROVIDER_URL");

        // Configuration values to be safely shared across requests.
        Arc::new(Configuration {
            env,
            listen_address,
            app_port,
            db_str,
            db_pool_max_size,
            rpc_url,
        })
    }

    // Helper function to set db connection string in test environment
    pub fn set_db_str(&mut self, db_str: String) {
        self.db_str = db_str
    }
}

impl FromStr for Environment {
    type Err = String;

    // String representation of Environment Variants.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "Invalid environment: {}. \"development\" or \"production\" currently supported",
                s
            )),
        }
    }
}

// Helper function to read environment variables
pub fn env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{}: {}", name, e))
        .expect("Missing environment variable")
}

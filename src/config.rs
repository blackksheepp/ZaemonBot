// src/config.rs
use log::error;
use std::env::var;

macro_rules! env_var {
    ($env_prefix:expr, $variable_name:expr) => {{
        match var(format!("{0}{1}", $env_prefix, $variable_name)) {
            Ok(val) => val,
            Err(_) => {
                error!("{0} not set. Please set the environment variable.", $variable_name);
                std::process::exit(1);
            }
        }
    }};
}

pub struct Config {
    pub mongodb_uri: String,
    pub db_name: String,
    pub bot_token: String,
    // Add more configuration variable_names as needed
}

impl Config {
    pub fn new() -> Self {
        

        dotenv::dotenv().ok(); // Load environment variable_names from .env file

        let env_prefix = var("ENV_PREFIX").unwrap_or(String::new());

        let mongodb_uri= env_var!(env_prefix, "MONGODB_URI");
        let db_name= env_var!(env_prefix, "DB_NAME");
        let bot_token= env_var!(env_prefix, "BOT_TOKEN");

        Config {
            mongodb_uri,
            db_name, 
            bot_token, 
        }
    }
}

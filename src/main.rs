// In src/main.rs or any other file

mod config;
mod database {
    pub mod db;
    pub mod models;
    pub mod user;
}
mod bot {
    pub mod client;
    pub mod inline_handler;
    pub mod message_handler;
    mod handler {
        pub mod inline;
        pub mod start;
    }
}
mod utils {
    pub mod image;
}


use bot::client;
use log::info;
use tokio;

#[tokio::main]
async fn main() {
    info!("Initializing configuration...");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    

    client::run().await;
}

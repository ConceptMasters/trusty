#![recursion_limit = "256"]

#[macro_use]
extern crate lazy_static;

mod errors;
mod race;
mod rob;
mod routes;
mod store;
mod validation;

use crate::routes::router;
use crate::store::{MongoStore, Store};
use log::{error, info};
use race::AccessControlEngine;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
struct Config {
    server_addr: SocketAddr,
    mongo_uri: String,
    mongo_db_name: String,
}

impl Config {
    fn from_env() -> Result<Self, env::VarError> {
        const DEFAULT_ADDR: &str = "0.0.0.0";
        const DEFAULT_PORT: &str = "3030";
        let mongo_uri = env::var("MONGO_URI")?;
        let mongo_db_name = env::var("MONGO_DB_NAME")?;
        let ip_address = env::var("SERVER_ADDR")
            .map(|a| {
                if a.is_empty() {
                    DEFAULT_ADDR.to_string()
                } else {
                    a
                }
            })
            .unwrap_or(DEFAULT_ADDR.to_string());
        let port = env::var("SERVER_PORT")
            .map(|p| {
                if p.is_empty() {
                    DEFAULT_PORT.to_string()
                } else {
                    p
                }
            })
            .unwrap_or(DEFAULT_PORT.to_string());
        let full_addr = format!("{}:{}", ip_address, port);
        let server_addr = full_addr.parse().map_err(|_| env::VarError::NotPresent)?;

        Ok(Self {
            server_addr,
            mongo_uri,
            mongo_db_name,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");

    let race_core = AccessControlEngine::init_with_mongo_store(
        config.mongo_uri.clone(),
        config.mongo_db_name.clone(),
    )
    .await
    .unwrap_or_else(|e| {
        error!(
            "Failed to initialize access control engine with Mongo store: {:?}",
            e
        );
        std::process::exit(1);
    });
    let access_control_interface: Arc<AccessControlEngine> = Arc::new(race_core);

    let mongo_store = MongoStore::init(config.mongo_uri.clone(), config.mongo_db_name.clone())
        .await
        .unwrap_or_else(|e| {
            error!(
                "failed to connect to MongoDB at URI '{}' with error: {:?}",
                config.mongo_uri, e
            );
            std::process::exit(1);
        });
    let store: Arc<dyn Store> = Arc::new(mongo_store.clone());
    let store_for_routes = store.clone();

    info!("Server starting at {}", config.server_addr);

    tokio::select! {
        _ = warp::serve(router(store_for_routes, access_control_interface)).run(config.server_addr) => {
            info!("Server shutting down...");
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-C received, shutting down...");
        }
    }
    Ok(())
}

use clap::{Arg, Command};
use tracing::{info, error};
use tracing_subscriber;

use std::io;

use jasper::types::{Label, Timestamp, DataPoint};
use jasper::storage::StorageEngine;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("jasper=debug,info")
        .init();

    info!("Starting Jasper");

    let matches = Command::new("jasper")
        .version(jasper::VERSION)
        .about("A distributed time series database for game sim")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .default_value("jasper.yaml")
        )
        .arg(
            Arg::new("node-id")
                .long("node-id")
                .value_name("ID")
                .help("Unique node identifier")
                .required(true)
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let node_id = matches.get_one::<String>("node-id").unwrap();

    info!("Loading configuration from: {}", config_path);
    info!("Node ID: {}", node_id);

    info!("🏰 Jasper server starting");

    let engine = StorageEngine::new("jasper.log");

    let dp = DataPoint {
        label: Label("npc_interaction".to_string()),
        timestamp: Timestamp(1686152130000000000),
        value: json!({
            "npc": "Bob",
            "action": "trade",
            "item": "sword"
        }),
    };

    // Write datapoint
    engine.write(&dp).unwrap();

    let read_back = engine.read_all().unwrap();
    println!("{:?}", read_back);

    // For now, just keep the process alive
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Jasper server");

    Ok(())

    // let db = StorageEngine::new("test.db");
    // db.write("hello")?;
    // let contents = db.read()?;
    // println!("{}", contents);
    // Ok(())
}

use clap::Parser;
use log::{debug, info};

use crate::watcher::Watcher;

mod config;
mod watcher;
mod sync;
mod source;
mod target;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Only run once and exit
    #[arg(short, long, default_value_t = false)]
    once: bool,

    /// Generate a default config file and exit
    #[arg(short, long, default_value_t = false)]
    generate_config: bool,

    /// Path to the config file
    #[arg(short, long, default_value = "config.yml")]
    config_path: String,

    /// Config as a json string
    #[arg(short, long, default_value = None)]
    raw_config: Option<String>,
}

#[tokio::main]
async fn main() {
    initialize_logger();

    let args = Args::parse();

    debug!("Starting with args: {:#?}", args);

    if args.generate_config {
        config::generate_default_config(&args.config_path);
        info!("Generated default config file at {}", args.config_path);
        return;
    }


    let config = match args.raw_config {
        Some(raw_config) => {
            info!("Using raw config");
            serde_json::from_str(&raw_config).expect("Failed to parse config")
        }
        None => {
            info!("Using config file at {}", args.config_path);
            config::load_config(&args.config_path)
        }
    };

    debug!("Config: {:#?}", config);

    if args.once {
        sync::sync(&config).await;
    } else {
        Watcher::new(config).run().await;
    }
}


fn initialize_logger() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}
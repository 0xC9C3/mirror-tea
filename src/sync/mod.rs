use std::collections::HashMap;

use log::info;

use crate::config::{Config, Source, SourceType, Target};
use crate::source;

pub async fn sync(config: &Config) {
    info!("Syncing");

    let targets = &config.targets;

    for (name, source) in &config.sources {
        info!("Syncing source: {}", name);

        match source.sync(targets).await {
            Ok(_) => info!("Synced source: {}", name),
            Err(err) => info!("Failed to sync source {}: {}", name, err),
        }
    }
}
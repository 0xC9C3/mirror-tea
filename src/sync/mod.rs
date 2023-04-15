use std::collections::HashMap;

use log::info;

use crate::config::{Config, Source, SourceType, Target};
use crate::source;

pub async fn sync(config: &Config) {
    info!("Syncing");

    let targets = &config.targets;

    for (name, source) in &config.sources {
        info!("Syncing source: {}", name);

        match sync_source(source, targets).await {
            Ok(_) => info!("Synced source: {}", name),
            Err(err) => info!("Failed to sync source {}: {}", name, err),
        }
    }
}

async fn sync_source(source: &Source, targets: &HashMap<String, Target>) -> anyhow::Result<()> {
    match source.type_ {
        SourceType::Github => source::github::sync(source, targets).await,
    }
}
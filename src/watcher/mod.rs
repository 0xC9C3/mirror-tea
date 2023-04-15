use log::info;
use tokio::time::Duration;

use crate::config::Config;
use crate::sync::sync;

pub struct Watcher {
    config: Config,
}

impl Watcher {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    pub async fn run(&mut self) {
        info!("Running watcher");

        loop {
            sync(&self.config).await;

            info!("Sleeping for {}ms", self.config.watcher_interval_ms);
            tokio::time::sleep(
                Duration::from_millis(self.config.watcher_interval_ms)
            ).await;
        }
    }
}
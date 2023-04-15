use std::collections::HashMap;

use crate::config::{Source, SourceType, Target};

pub mod github;

impl Source {
    pub async fn sync(&self, targets: &HashMap<String, Target>) -> anyhow::Result<()> {
        match self.type_ {
            SourceType::Github => self.sync_from_github(targets).await,
        }
    }
}
use crate::config::{Source, SourceType, Target, TargetType};

pub mod gitea;

impl Target {
    pub async fn mirror(&self, name: String, url: String, source: &Source) -> anyhow::Result<()> {
        match self.type_ {
            TargetType::Gitea => self.mirror_to_gitea(name, url, source).await,
        }
    }
}
use crate::config::{Target, TargetType};

pub mod gitea;

impl Target {
    pub async fn mirror(&self, name: String, url: String, token: String) -> anyhow::Result<()> {
        match self.type_ {
            TargetType::Gitea => self.mirror_to_gitea(name, url, token).await,
        }
    }
}
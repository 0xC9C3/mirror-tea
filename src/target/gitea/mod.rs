use gitea_rs::apis::repository_api::repo_migrate;
use gitea_rs::apis::user_api;
use gitea_rs::models::migrate_repo_options::Service;
use gitea_rs::models::MigrateRepoOptions;
use log::info;

use crate::config::{Source, SourceType, Target};

impl Target {
    pub async fn mirror_to_gitea(&self, name: String, url: String, source: &Source) -> anyhow::Result<()> {
        let mut config = gitea_rs::apis::configuration::Configuration::default();
        config.base_path = self.url.clone();
        config.basic_auth = Some((self.auth.username.clone(), Some(self.auth.password.clone())));


        // check if repo already exists
        let mut repos = vec![];

        let mut page = 1;
        let limit = 15;
        loop {
            let resp = user_api::user_current_list_repos(
                &config,
                Some(page),
                Some(limit),
            ).await?;

            let resp_len = resp.len();
            repos.extend(resp);

            if resp_len < limit as usize {
                break;
            }

            page += 1;
        }

        if repos.iter().any(|r| r.name == Some(name.clone())) {
            info!("Repo {} already exists on target", name);
            return Ok(());
        }

        let service = match source.type_ {
            SourceType::Github => Service::Github,
        };

        let body = MigrateRepoOptions {
            repo_name: name,
            clone_addr: url,
            auth_token: Some(source.token.clone()),
            mirror: Some(true),
            service: Some(service),
            auth_password: None,
            auth_username: None,
            description: None,
            issues: Some(self.issues),
            labels: Some(self.labels),
            lfs: None,
            lfs_endpoint: None,
            milestones: Some(self.milestones),
            mirror_interval: Some(self.mirror_interval.clone()),
            private: Some(self.private),
            pull_requests: Some(self.pull_requests),
            releases: Some(self.releases),
            wiki: Some(self.wiki),
            repo_owner: None,
            uid: None,
        };

        repo_migrate(&config, Some(body)).await?;

        Ok(())
    }
}


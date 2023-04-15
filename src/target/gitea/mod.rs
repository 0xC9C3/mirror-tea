use gitea_rs::apis::{repository_api, user_api};
use gitea_rs::apis::configuration::{ApiKey, BasicAuth};
use gitea_rs::apis::repository_api::repo_migrate;
use gitea_rs::models::migrate_repo_options::Service;
use gitea_rs::models::MigrateRepoOptions;
use log::info;

use crate::config::Target;

pub async fn create_repo(target: &Target, name: String, url: String, token: String) -> anyhow::Result<()> {
    let mut config = gitea_rs::apis::configuration::Configuration::default();
    config.base_path = target.url.clone();
    config.basic_auth = Some((target.auth.username.clone(), Some(target.auth.password.clone())));


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

    let mut body = MigrateRepoOptions {
        repo_name: name,
        clone_addr: url,
        auth_token: Some(token),
        mirror: Some(true),
        service: Some(Service::Github),
        auth_password: None,
        auth_username: None,
        description: None,
        issues: Some(target.issues),
        labels: Some(target.labels),
        lfs: None,
        lfs_endpoint: None,
        milestones: Some(target.milestones),
        mirror_interval: Some(target.mirror_interval.clone()),
        private: Some(target.private),
        pull_requests: Some(target.pull_requests),
        releases: Some(target.releases),
        wiki: Some(target.wiki),
        repo_owner: None,
        uid: None,
    };

    repo_migrate(&config, Some(body)).await?;

    Ok(())
}


use std::collections::HashMap;

use log::{info, warn};
use octocrab::{Octocrab, Page};
use octocrab::models::Repository;

use crate::config::{Source, Target};
use crate::target;

pub async fn sync(source: &Source, targets: &HashMap<String, Target>) -> anyhow::Result<()> {
    let client = Octocrab::builder()
        .personal_token(source.token.clone())
        .build()?;

    let page = if let Some(org_name) = source.org.clone() {
        client
            .orgs(org_name)
            .list_repos()
            .sort(octocrab::params::repos::Sort::Created)
            .send()
            .await?
    } else {
        client.current()
            .list_repos_for_authenticated_user()
            .sort("created")
            .send()
            .await?
    };

    let repos = iterate_repositories(&client, page).await?;

    for repo in repos {
        for (target_name, target) in targets {
            let clone_url = match repo.clone_url {
                None => {
                    warn!("Skipping repo {} because it has no clone url", repo.name);
                    continue;
                }
                _ => repo.clone_url.clone().unwrap().as_str().to_string(),
            };

            let creation_result = target::gitea::create_repo(
                target,
                repo.name.clone(),
                clone_url,
                source.token.clone(),
            ).await;

            if creation_result.is_err() {
                warn!("Failed to create repo {} on target {}: {}", repo.name, target_name, creation_result.err().unwrap());
            }
        }
    }

    Ok(())
}

async fn iterate_repositories(client: &Octocrab, page: Page<Repository>) -> anyhow::Result<Vec<Repository>> {
    let mut new_repos = Vec::new();
    let mut next_page = page;

    loop {
        for repo in &next_page {
            new_repos.push(repo.clone());
        }

        next_page = match client
            .get_page::<Repository>(&next_page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    Ok(
        new_repos
    )
}
# Mirror tea

Mirror tea is a small tool to automatically mirror all git repositories to a gitea instance.

Currently, it only supports mirroring from GitHub to gitea.

PRs are welcome.

## Configuration

```yaml
watcher_interval_ms: 30000
targets:
  gitea_local:
    type: Gitea
    url: https://your.gitea.instance/api/v1
    issues: true
    labels: true
    milestones: true
    mirror_interval: 8h0m0s
    private: true
    pull_requests: true
    releases: true
    wiki: true
    auth:
      username: your_username
      password: your_password
sources:
  github_personal:
    type: Github
    token: your_github_token
    targets:
      - gitea_local
```

## Docker & Docker Compose

## Bare Metal



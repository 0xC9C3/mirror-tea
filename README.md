# Mirror tea

[![Docker](https://img.shields.io/badge/dockerhub-image-success.svg?logo=Docker)](https://hub.docker.com/r/0xc9c3/mirror-tea)

Mirror tea is a small tool to automatically mirror all git repositories to a gitea instance.

Currently, it only supports mirroring from GitHub to gitea.

PRs are welcome.

## Configuration

The configuration consists of two parts: the sources and the targets.

### Sources

Sources are the places where the repositories are mirrored from.

<details>
<summary>Properties</summary>

#### type

The type of the source. Currently only "Github" is supported.

#### token

The token to authenticate with the source.

#### targets

The targets where the repositories should be mirrored to.

#### org

The organization to mirror. If not set, all repositories of the user are mirrored.

</details>

### Targets

Targets are the places where the repositories are mirrored to.

<details>
<summary>Properties</summary>

#### type

The type of the target. Currently only "Gitea" is supported.

#### url

The url of the target.

#### issues

Mirror issues.

#### labels

Mirror labels.

#### milestones

Mirror milestones.

#### mirror_interval

The interval in which the repositories are mirrored.

#### private

Should the repository be private.

#### pull_requests

Mirror pull requests.

#### releases

Mirror releases.

#### wiki

Mirror wiki.

#### auth

Authentication information for the target. Only username and password are supported.

</details>

Example

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

## Parameters

### once -o

Run the application once and exit.

### generate_config -g

Generate a config.yml file in the current directory.

### config_path -c

Path to the config.yml file.

### raw_config -r

Provide the config as a json string.

### help -h

Print the help message.

## Docker & Docker Compose

https://hub.docker.com/r/0xc9c3/mirror-tea

### Example docker-compose.yml

```yaml
version: '3'

services:
  mirror:
    image: 0xc9c3/mirror-tea
    container_name: mirror
    restart: unless-stopped
    volumes:
      - ./config.yml:/config.yml
    command: -c /config.yml
```

## Bare Metal

Build the application using cargo:

```bash
cargo build --release
```

Run the application:

```bash
./target/release/mirror-tea -c config.yml
```

You also have the option to use i.e. cron to run the application periodically using the "once" parameter.

```bash
0 0 * * * /path/to/mirror-tea -o -c /path/to/config.yml
```

use git2::build::RepoBuilder;
use git2::{Cred, RemoteCallbacks, Repository};
use serde::Deserialize;
use shellexpand::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Remote {
    base_addr: String,
    name: String,
}

#[derive(Deserialize)]
struct Base {
    #[serde(rename = "workspace")]
    configs: Vec<Config>,
}

#[derive(Deserialize)]
struct Config {
    path: String,
    ssh_key_path: String,
    repos: Vec<String>,
    origin: Remote,
    remotes: Vec<Remote>,
}

struct Workspace {
    workspace_path: PathBuf,
    ssh_key_path: String,
    repos: Vec<String>,
    origin: Remote,
    remotes: Vec<Remote>,
}

impl Workspace {
    fn from_config(config: Config) -> Workspace {
        let path = env(&config.path).unwrap().to_string();
        let path = PathBuf::from(&path);

        Workspace {
            workspace_path: path,
            ssh_key_path: config.ssh_key_path,
            origin: config.origin,
            repos: config.repos,
            remotes: config.remotes,
        }
    }

    fn setup(&self) {
        if !&self.workspace_path.exists() {
            fs::create_dir_all(&self.workspace_path).expect("couldn't create workspace path");
        }

        for repo_name in &self.repos {
            let repo = match Repository::open(self.workspace_path.join(repo_name)) {
                Ok(repo) => repo,
                Err(_) => self.clone(repo_name),
            };

            self.connect_remotes(repo, repo_name);
        }
    }

    fn repo_builder(&self) -> RepoBuilder {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.expect("couldn't parse username"),
                None,
                Path::new(&env(&self.ssh_key_path).unwrap().to_string()),
                None,
            )
        });

        let mut fetch_opts = git2::FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        builder
    }

    fn clone(&self, repo_name: &String) -> Repository {
        let mut builder = self.repo_builder();
        let repo = builder
            .clone(
                &format!("{}/{}", self.origin.base_addr, repo_name),
                &self.workspace_path.join(repo_name),
            )
            .expect(&format!("couldn't clone repo: {}", repo_name));

        if &self.origin.name != "origin" {
            repo.remote_rename("origin", &self.origin.name).unwrap();
        }

        repo
    }

    fn connect_remotes(&self, repo: Repository, repo_name: &String) {
        for remote in &self.remotes {
            match repo.find_remote(&remote.name) {
                Ok(_) => (),
                Err(_) => {
                    repo.remote(
                        &remote.name,
                        &format!("{}/{}", &remote.base_addr, repo_name),
                    )
                    .expect(&format!(
                        "couldn't create remote {} on repo {}",
                        remote.name, repo_name
                    ));
                }
            };
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("no config path provided");
    let config_contents = fs::read_to_string(path).expect("couldn't read the file contents");
    let base: Base = toml::from_str(&config_contents).expect("couldn't parse the file as toml");

    for config in base.configs {
        let workspace = Workspace::from_config(config);
        workspace.setup();
    }
}

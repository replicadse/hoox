use std::{
    collections::HashMap,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

pub const GIT_HOOK_NAMES: [&str; 19] = [
    "applypatch-msg",
    "commit-msg",
    "post-applypatch",
    "post-checkout",
    "post-commit",
    "post-merge",
    "post-receive",
    "post-rewrite",
    "post-update",
    "pre-applypatch",
    "pre-auto-gc",
    "pre-commit",
    "pre-push",
    "pre-rebase",
    "pre-receive",
    "prepare-commit-msg",
    "push-to-checkout",
    "sendemail-validate",
    "update",
];

pub async fn init_hooks_files(cwd: &PathBuf) -> anyhow::Result<()> {
    let perms = std::fs::Permissions::from_mode(0o755);
    for hook_name in GIT_HOOK_NAMES {
        let hook_path = cwd.join(".git/hooks").join(&hook_name);
        std::fs::write(&hook_path, "hoox run --hook=${0##*/}")?;
        std::fs::set_permissions(&hook_path, perms.clone())?;
    }
    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WithVersion {
    pub version: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hoox {
    pub version: String,
    pub verbosity: Option<Verbosity>,
    pub hooks: HashMap<String, Vec<Command>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Command {
    pub program: Option<Vec<String>>,
    pub severity: Option<CommandSeverity>,
    pub command: String,
    pub verbosity: Option<Verbosity>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verbosity {
    All,
    None,
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSeverity {
    Error,
    Warn,
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_ser_hoox() {
        let hoox = Hoox {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            verbosity: Some(Verbosity::All),
            hooks: HashMap::from_iter(GIT_HOOK_NAMES.iter().map(|hook_name| {
                (hook_name.to_string(), vec![Command {
                    program: Some(vec!["sh", "-c"].iter().map(|v| v.to_string()).collect::<Vec<_>>()),
                    command: "echo 'Hello, world!'".to_owned(),
                    severity: Some(CommandSeverity::Warn),
                    verbosity: None,
                }])
            })),
        };
        println!("{}", serde_yaml::to_string(&hoox).unwrap());
    }
}

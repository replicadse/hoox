use std::path::{
    Path,
    PathBuf,
};

use anyhow::Result;

use crate::schema;

const HOOX_FILE_NAME: &str = ".hoox.yaml";

pub fn get_repo_path(mut cwd: PathBuf) -> Result<PathBuf> {
    while std::fs::read_dir(cwd.join(".git")).is_err() {
        if !cwd.pop() {
            return Err(anyhow::anyhow!("not a git repository"));
        }
    }
    Ok(cwd)
}

pub async fn init(repo_path: &Path) -> Result<()> {
    let hoox_path = repo_path.join(HOOX_FILE_NAME);
    if std::fs::read_to_string(&hoox_path).is_err() {
        std::fs::write(
            &hoox_path,
            format!(
                r#"version: "{}"
verbosity: all

# Available Git hooks:
# - {}

{}
"#,
                env!("CARGO_PKG_VERSION"),
                schema::GIT_HOOK_NAMES.join(" \n# - "),
                include_str!("../res/templates/rust.yaml"),
            ),
        )?;
    }
    schema::init_hooks_files(repo_path).await?;
    Ok(())
}

#[allow(unused)]
pub async fn run(hook: &str, args: &Vec<String>, ignore_missing: bool) -> Result<()> {
    let cwd = get_repo_path(std::env::current_dir()?)?;
    let hoox_path = cwd.join(HOOX_FILE_NAME);

    let file_content = std::fs::read_to_string(&hoox_path)?;
    let version = serde_yaml::from_str::<schema::WithVersion>(&file_content)?;
    let file_v = version.version.split('.').collect::<Vec<_>>();
    let cli_v = env!("CARGO_PKG_VERSION").split('.').collect::<Vec<_>>();
    if version_compare::compare(&version.version, env!("CARGO_PKG_VERSION")).unwrap() == version_compare::Cmp::Gt {
        return Err(anyhow::anyhow!("hoox version is outdated, please update"));
    }
    if file_v[0] == "0" && cli_v[0] == "0" {
        if file_v[1] != cli_v[1] {
            return Err(anyhow::anyhow!(
                "hoox minor version is incompatible (needs to be same below 1.0.0)"
            ));
        }
    } else if file_v[0] != cli_v[0] {
        return Err(anyhow::anyhow!("hoox major version is incompatible"));
    }

    let hoox = serde_yaml::from_str::<schema::Hoox>(&file_content)?;
    let verbosity = &hoox.verbosity.unwrap_or(schema::Verbosity::All);

    if let Some(commands) = hoox.hooks.get(hook) {
        for command in commands {
            let program = command.program.clone().or_else(|| Some(vec!["sh".to_owned(), "-c".to_owned()])).unwrap();
            if program.is_empty() {
                return Err(anyhow::anyhow!("can not execute empty program for {}", hook));
            }
            let mut exec = &mut std::process::Command::new(&program[0]);
            exec = exec.args(program.iter().skip(1).collect::<Vec<_>>());
            exec = match command.command {
                | schema::CommandContent::Inline(ref content) => exec.arg(content),
                | schema::CommandContent::File(ref file) => exec.arg(std::fs::read_to_string(cwd.join(file))?),
            };
            exec = exec.arg(&hoox_path).args(args);
            let output = exec.output()?;

            let verbosity = command.verbosity.clone().unwrap_or(verbosity.clone());
            if verbosity == schema::Verbosity::All || verbosity == schema::Verbosity::Stdout {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.len() > 0 {
                    println!("{}", stdout);
                }
            }
            if verbosity == schema::Verbosity::All || verbosity == schema::Verbosity::Stderr {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.len() > 0 {
                    eprintln!("{}", stderr);
                }
            }

            if command.severity.is_none() || command.severity == Some(schema::CommandSeverity::Error) {
                let status = exec.status().unwrap();
                if !status.success() {
                    return Err(anyhow::anyhow!("hook failed with code {}", status.code().unwrap()));
                }
            }
        }
    } else if !ignore_missing {
        return Err(anyhow::anyhow!("hook not found"));
    }
    Ok(())
}

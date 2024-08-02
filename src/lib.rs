pub mod schema;

use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
use schema::Verbosity;

const HOOX_FILE_NAME: &'static str = ".hoox.yaml";

pub async fn init(mut cwd: PathBuf) -> Result<()> {
    while std::fs::read_dir(cwd.join(".git")).is_err() {
        dbg!(&cwd);
        if !cwd.pop() {
            return Err(anyhow::anyhow!("not a git repository"));
        }
    }
    let hoox_path = cwd.join(HOOX_FILE_NAME);

    if let Err(_) = std::fs::read_to_string(&hoox_path) {
        std::fs::write(
            &hoox_path,
            format!(
                r#"version: "{}"

# Available Git hooks:
# - {}

# anchors
.cargo_fmt_check: &cargo_fmt_check |-
  cargo +nightly fmt --all -- --check
.cargo_test: &cargo_test |-
  cargo test --all

hooks:
  "pre-commit": # pre-commit hook
    - command: *cargo_fmt_check # re-use anchor
    - command: *cargo_test
    - command: 'cargo doc --no-deps'
      verbosity: stderr # [all, none, stdout, stderr]
      severity: warn # [error, warn]
  "pre-push": # pre-push hook
    - command: *cargo_fmt_check
    - command: *cargo_test

"#,
                env!("CARGO_PKG_VERSION"),
                schema::GIT_HOOK_NAMES.join(" \n# - ")
            ),
        )?;
    }
    schema::init_hooks_files(&cwd).await?;
    Ok(())
}

pub async fn run(hook: &str) -> Result<()> {
    let mut cwd = std::env::current_dir()?;
    while std::fs::read_dir(cwd.join(".git")).is_err() {
        if !cwd.pop() {
            return Err(anyhow::anyhow!("not a git repository"));
        }
    }
    let hoox_path = cwd.join(HOOX_FILE_NAME);

    let file_content = std::fs::read_to_string(hoox_path)?;
    let version = serde_yaml::from_str::<schema::WithVersion>(&file_content)?;
    let version_check = version_compare::compare(&version.version, env!("CARGO_PKG_VERSION")).unwrap();
    if version_check == version_compare::Cmp::Gt {
        return Err(anyhow::anyhow!("hoox version is outdated, please update"));
    }
    let hoox = serde_yaml::from_str::<schema::Hoox>(&file_content)?;
    let verbosity = &hoox.verbosity.unwrap_or(Verbosity::All);

    if let Some(hook) = hoox.hooks.get(hook) {
        for command in hook {
            let program = command.program.clone().or_else(|| Some(vec!["sh".to_owned(), "-c".to_owned()])).unwrap();
            let mut exec = &mut std::process::Command::new(&program[0]);
            exec = exec.args(program.iter().skip(1).collect::<Vec<_>>()).arg(&command.command);
            let output = exec.output()?;

            let verbosity = command.verbosity.clone().unwrap_or(verbosity.clone());
            if verbosity == Verbosity::All || verbosity == Verbosity::Stdout {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.len() > 0 {
                    println!("{}", stdout);
                }
            }
            if verbosity == Verbosity::All || verbosity == Verbosity::Stderr {
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
    }
    Ok(())
}

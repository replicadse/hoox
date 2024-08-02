pub mod schema;

use anyhow::Result;

const HOOX_FILE_NAME: &'static str = ".hoox.yaml";

pub async fn init() -> Result<()> {
    let mut cwd = std::env::current_dir()?;
    while std::fs::read_dir(cwd.join(".git")).is_err() {
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

hooks:
  "pre-commit":
    command: |-
      cargo +nightly fmt --all -- --check
  # "pre-commit":
  #   program: ["python", "-c"]
  #   command: |-
  #     print('executing hook')
  #     print('calling python program')
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

    if let Some(hook) = hoox.hooks.get(hook) {
        let program = hook.program.clone().or_else(|| Some(vec!["sh".to_owned(), "-c".to_owned()])).unwrap();
        let mut exec = &mut std::process::Command::new(&program[0]);
        exec = exec.args(program.iter().skip(1).collect::<Vec<_>>()).arg(&hook.command);
        let output = exec.output()?;
        if exec.status().unwrap().success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        if hook.severity == Some(schema::CommandSeverity::Error) {
            let status = exec.status().unwrap();
            if !status.success() {
                return Err(anyhow::anyhow!("hook failed with code {}", status.code().unwrap()));
            }
        }
    }
    Ok(())
}

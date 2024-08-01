pub mod schema;

use anyhow::Result;

const HOOX_DEF_PATH: &'static str = "./.hoox.toml";

pub async fn init() -> Result<()> {
    if let Err(_) = std::fs::read_to_string(HOOX_DEF_PATH) {
        std::fs::write(
            HOOX_DEF_PATH,
            format!(
                r#"version = "{}"

# Available Git hooks:
# - {}

[hooks.pre-commit]
command = "cargo +nightly fmt --all --check"

# Examples:

# [hooks.pre-commit]
# program = ["python", "-c"]
# command = """
# print('executing hook')
# print('calling python program')
# """
"#,
                env!("CARGO_PKG_VERSION"),
                schema::GIT_HOOK_NAMES.join(" \n# - ")
            ),
        )?;
    }
    schema::init_hooks_files().await?;
    Ok(())
}

pub async fn run(hook: &str) -> Result<()> {
    let hoox = toml::from_str::<schema::Hoox>(&std::fs::read_to_string(HOOX_DEF_PATH)?)?;
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
        if !exec.status().unwrap().success() {
            std::process::exit(1);
        }
    }
    Ok(())
}

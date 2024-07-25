include!("check_features.rs");

pub mod args;
pub mod reference;
pub mod schema;

use std::path::PathBuf;

use anyhow::Result;
use args::ManualFormat;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = crate::args::ClapArgumentLoader::load()?;

    match cmd.command {
        | crate::args::Command::Manual { path, format } => {
            let out_path = PathBuf::from(path);
            std::fs::create_dir_all(&out_path)?;
            match format {
                | ManualFormat::Manpages => {
                    reference::build_manpages(&out_path)?;
                },
                | ManualFormat::Markdown => {
                    reference::build_markdown(&out_path)?;
                },
            }
            Ok(())
        },
        | crate::args::Command::Autocomplete { path, shell } => {
            let out_path = PathBuf::from(path);
            std::fs::create_dir_all(&out_path)?;
            reference::build_shell_completion(&out_path, &shell)?;
            Ok(())
        },
        | crate::args::Command::Init => {
            if let Err(_) = std::fs::read_to_string("./.hoox.toml") {
                std::fs::write(
                    "./.hoox.toml",
                    r#"version = "0.0.0"

[hooks.pre-commit]
command = "cargo +nightly fmt --all --check"
"#,
                )?;
            }
            schema::init_hooks_files().await?;
            Ok(())
        },
        | crate::args::Command::Run { hook } => {
            let hoox = toml::from_str::<schema::Hoox>(&std::fs::read_to_string(".hoox.toml")?)?;
            if let Some(hook) = hoox.hooks.get(&hook) {
                let program = hook.program.clone().or_else(|| Some(vec!["sh".to_owned(), "-c".to_owned()])).unwrap();
                let output = std::process::Command::new(&program[0])
                    .args(program.iter().skip(1).collect::<Vec<_>>())
                    .arg(&hook.command)
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Ok(())
        },
    }
}

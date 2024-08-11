mod args;
mod commands;
mod reference;
mod schema;

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
        | crate::args::Command::Init { template } => {
            let template_content = match template {
                | args::InitTemplate::Rust => include_str!("../res/templates/rust.yaml"),
            };
            commands::init(
                &commands::get_repo_path(std::env::current_dir()?)?,
                Some(template_content),
            )
            .await?;
            Ok(())
        },
        | crate::args::Command::Run {
            hook,
            args,
            ignore_missing,
        } => {
            commands::run(&hook, &args, ignore_missing).await?;
            Ok(())
        },
    }
}

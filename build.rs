#[path = "src/commands.rs"]
mod commands;
#[path = "src/schema.rs"]
mod schema;

use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    if ci_info::is_ci() {
        return Ok(());
    };

    let dir = std::env::var("OUT_DIR")?;
    let cwd = PathBuf::from_str(&dir)?;
    if let Ok(repo) = commands::get_repo_path(cwd) {
        commands::init(&repo).await?;
    } else {
        eprintln!("not a git repository - skipping hook initialization");
    }
    Ok(())
}

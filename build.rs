#[path = "src/lib.rs"]
mod hoox;

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
    if let Ok(repo) = hoox::get_repo_path(cwd) {
        hoox::init(&repo).await?;
    } else {
        eprintln!("not a git repository - skipping hook initialization");
    }
    Ok(())
}

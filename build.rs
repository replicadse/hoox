#[path = "src/lib.rs"]
mod hoox;

use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let dir = std::env::var("OUT_DIR")?;

    if ci_info::is_ci() {
        return Ok(());
    };

    hoox::init(PathBuf::from_str(&dir)?).await.unwrap();
    Ok(())
}

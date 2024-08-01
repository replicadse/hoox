#[path = "src/lib.rs"]
mod hoox;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    if ci_info::is_ci() {
        return Ok(());
    };

    hoox::init().await.unwrap();
    Ok(())
}

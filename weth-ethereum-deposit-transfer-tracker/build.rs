use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("weth", "abis/weth.json")?
        .generate()?
        .write_to_file("src/abis/weth.rs")?;
    Ok(())
}
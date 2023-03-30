use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc721_non_standard", "abis/erc721_non_std.json")?
        .generate()?
        .write_to_file("src/abis/erc721_non_standard.rs")?;
    Ok(())
}

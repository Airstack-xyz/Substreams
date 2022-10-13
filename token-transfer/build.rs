use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc20", "abis/erc20.json")?
        .generate()?
        .write_to_file("src/abis/erc20.rs")?;
    Abigen::new("erc721", "abis/erc721.json")?
        .generate()?
        .write_to_file("src/abis/erc721.rs")?;
    Abigen::new("erc1155", "abis/erc1155.json")?
        .generate()?
        .write_to_file("src/abis/erc1155.rs")?;
    Ok(())
}
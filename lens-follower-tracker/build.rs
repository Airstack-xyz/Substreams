use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("LensLPPEvents", "abis/lens_events.json")?
        .generate()?
        .write_to_file("src/abis/lens_events.rs")?;
    Abigen::new("erc721", "abis/erc721_events.json")?
    .generate()?
    .write_to_file("src/abis/erc721_events.rs")?;
    Ok(())
}
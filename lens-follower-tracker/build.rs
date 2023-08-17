use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("LensLPPEvents", "abis/lens_events.json")?
        .generate()?
        .write_to_file("src/abis/lens_events.rs")?;
    Ok(())
}
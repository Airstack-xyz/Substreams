use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("lenslpp", "abis/lenslpp.json")?
        .generate()?
        .write_to_file("src/abis/lenslpp.rs")?;
    Ok(())
}
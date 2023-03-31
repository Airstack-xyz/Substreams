use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new(
        "non_standard_transaction",
        "abis/non_standard_transaction.json",
    )?
    .generate()?
    .write_to_file("src/abis/non_standard_transaction.rs")?;
    Ok(())
}

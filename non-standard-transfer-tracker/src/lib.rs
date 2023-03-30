mod abis;
mod helpers;
mod pb;

use pb::transfer_tracker::{Transfer, Transfers};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        // log::info!("Tx Hash {}", tx_hash);

        // ERC721 Transfer
        if let Some(event) = abis::ERC721_non_standard::events::Transfer::match_and_decode(log) {
            log::info!(
                "ERC721 non std {}",
                Hex(log.receipt.transaction.hash.clone()).to_string()
            );

            let erc721_transfer: Transfer = Transfer {
                amount: event.token_id.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.clone().address()).to_string(),
                ),
                chain_id: 137.to_string(),
                log_index: log.block_index(),
                source: 1,
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                token_id: event.token_id.to_string(),
                operator: helpers::utils::format_with_0x(
                    Hex(log.receipt.transaction.from.clone()).to_string(),
                ),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 0, // token type will be detected outside this substream by token tracker substream
                block_number: blk.number,
                block_timestamp: blk
                    .clone()
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64,
                // log_ordinal: log.ordinal(),
                ..Default::default()
            };
            transfers.push(erc721_transfer);
        }
    }
    Ok(Transfers { transfers })
}

mod abis;
mod helpers;
mod pb;

use pb::transfer_tracker::{Erc1155TransferBatch, Erc1155TransferSingle, Transfer, Transfers};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    let mut erc1155_transfer_batchs = vec![];
    let mut erc1155_transfer_singles = vec![];
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        log::info!("Tx Hash {}", tx_hash);

        // ERC20 Transfer
        if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log) {
            let erc20_transfer: Transfer = Transfer {
                amount: event.value.to_string(),
                token_id: event.value.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.clone().address()).to_string(),
                ),
                chain_id: 137.to_string(),
                log_index: log.block_index(),
                source: 1,
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                operator: helpers::utils::format_with_0x(
                    Hex(log.receipt.transaction.from.clone()).to_string(),
                ),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 0,
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
            transfers.push(erc20_transfer);
        }

        // ERC721 Transfer
        if let Some(event) = abis::erc721::events::Transfer::match_and_decode(log) {
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

        // ERC1155 Single
        if let Some(event) = abis::erc1155::events::TransferBatch::match_and_decode(log) {
            let erc1155_transfer_batch: Erc1155TransferBatch = Erc1155TransferBatch {
                amounts: event.values.iter().map(|c| c.clone().to_string()).collect(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index(),
                source: 1,
                chain_id: 137.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.log.clone().address).to_string(),
                ),
                operator: helpers::utils::format_with_0x(Hex(event.operator).to_string()),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 3,
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
                token_ids: event
                    .ids
                    .iter()
                    .map(|id| Hex(id.clone()).0.to_string())
                    .collect(),
                // log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_batchs.push(erc1155_transfer_batch);
        }

        // ERC1155 Batch
        if let Some(event) = abis::erc1155::events::TransferSingle::match_and_decode(log) {
            let erc1155_transfer_single: Erc1155TransferSingle = Erc1155TransferSingle {
                amount: event.value.to_string(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index(),
                source: 1,
                chain_id: 137.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.log.clone().address).to_string(),
                ),
                operator: helpers::utils::format_with_0x(Hex(event.operator).to_string()),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 3,
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
                token_id: event.id.to_string(),
                // log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_singles.push(erc1155_transfer_single);
        }
    }
    Ok(Transfers {
        transfers,
        erc1155_transfer_batchs,
        erc1155_transfer_singles,
    })
}

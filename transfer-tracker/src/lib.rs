mod abis;
mod helpers;
mod pb;

use anyhow::anyhow;
use pb::transfer_tracker::{Erc1155TransferBatch, Erc1155TransferSingle, Transfer, Transfers};
use substreams::errors::Error;
use serde::Deserialize;
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[derive(Deserialize)]
struct InputFilters {
    weth_contract_address: Option<String>,
}

#[substreams::handlers::map]
fn map_transfers(params: String, blk: eth::Block) -> Result<Transfers, Vec<Error>> {
    let mut transfers = vec![];
    let mut erc1155_transfer_batchs = vec![];
    let mut erc1155_transfer_singles = vec![];
    let filters: InputFilters = parse_filters_from_params(params)?;
    let weth_contract_address: String = filters.weth_contract_address.expect("weth_contract_address is already verified");
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
                chain_id: 1.to_string(),
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
                chain_id: 1.to_string(),
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
                chain_id: 1.to_string(),
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
                chain_id: 1.to_string(),
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
        
        let token_address = helpers::utils::format_with_0x(
            Hex(log.clone().address()).to_string(),
        );
        
        if token_address == weth_contract_address.to_string(){
            if let Some(event) = abis::weth::events::Deposit::match_and_decode(log) {
                let erc20_transfer: Transfer = Transfer {
                    amount: event.wad.to_string(),
                    token_id: event.wad.to_string(),
                    token_address: helpers::utils::format_with_0x(
                        Hex(log.clone().address()).to_string(),
                    ),
                    chain_id: 1.to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                    operator: helpers::utils::format_with_0x(
                        Hex(log.receipt.transaction.from.clone()).to_string(),
                    ),
                    from: helpers::constants::ZERO_ADDRESS.to_string(),
                    to: helpers::utils::format_with_0x(Hex(event.dst).to_string()),
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
    
            if let Some(event) = abis::weth::events::Withdrawal::match_and_decode(log) {
                let erc20_transfer: Transfer = Transfer {
                    amount: event.wad.to_string(),
                    token_id: event.wad.to_string(),
                    token_address: helpers::utils::format_with_0x(
                        Hex(log.clone().address()).to_string(),
                    ),
                    chain_id: 1.to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                    operator: helpers::utils::format_with_0x(
                        Hex(log.receipt.transaction.from.clone()).to_string(),
                    ),
                    from: helpers::utils::format_with_0x(Hex(event.src).to_string()),
                    to: helpers::constants::ZERO_ADDRESS.to_string(),
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
        }
        // Deposit Transfer
    }
    Ok(Transfers {
        transfers,
        erc1155_transfer_batchs,
        erc1155_transfer_singles,
    })
}

fn parse_filters_from_params(
    params: String,
) -> Result<InputFilters, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params);
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!(
            "Unexpected error while parsing parameters"
        )]));
    }

    let filters = parsed_result.unwrap();
    verify_filters(&filters)?;

    Ok(filters)
}

fn verify_filters(params: &InputFilters) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    let weth_contract_address = params.weth_contract_address.as_ref().unwrap();
    if !helpers::utils::is_address_valid(weth_contract_address) {
        errors.push(anyhow!(
            "'weth_contract_address' address ({}) is not valid",
            weth_contract_address
        ));
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(())
}
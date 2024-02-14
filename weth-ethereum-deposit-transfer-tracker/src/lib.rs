mod abis;
mod helpers;
mod pb;

use anyhow::anyhow;
use pb::transfer_tracker::{ Transfer, Transfers};
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
    let filters: InputFilters = parse_filters_from_params(params)?;
    let weth_contract_address: String = filters.weth_contract_address.expect("weth_contract_address is already verified");
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        log::info!("Tx Hash {}", tx_hash);

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
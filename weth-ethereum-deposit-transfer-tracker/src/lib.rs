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
        log::info!("Tx Hash {}", tx_hash);
        let token_address = helpers::utils::format_with_0x(
            Hex(log.clone().address()).to_string(),
        );
        //wETH Ethereum Address
        if token_address == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(){
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
                    from: String::from("0x0000000000000000000000000000000000000000"),
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
                    to: String::from("0x0000000000000000000000000000000000000000"),
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

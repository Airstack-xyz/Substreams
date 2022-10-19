mod abis;
mod pb;
mod helpers;

use pb::transfer_tracker::{Transfers, Transfer, Erc1155TransferBatch, Erc1155TransferSingle};
use substreams::store;
use substreams::{log, Hex, proto};
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use helpers::constant:: {BASE_TOKEN_ADDRESS};

use crate::helpers::utils::bigint_from_bytes;

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    let mut erc1155_transfer_batchs = vec![];
    let mut erc1155_transfer_singles = vec![];
    for trx in blk.transactions(){
        log::info!("Tx Hash {}", Hex(trx.hash.clone()).to_string());
        for call in trx.calls() {
            // ETH Transfer Tracking
            match &call.call.value {
                Some(value) => {
                    let formatted_value = bigint_from_bytes(value.bytes.clone()).to_string();
                    
                    let base_token_transfer: Transfer = Transfer{
                        transaction_hash:Hex(trx.hash.clone()).to_string(),
                        call_index: call.call.index,
                        call_depth: call.call.depth,
                        source: 0,
                        chain_id:1.to_string(),
                        token_address: BASE_TOKEN_ADDRESS.to_string(),
                        operator:Hex(call.call.caller.clone()).to_string(),
                        from: Hex(call.call.caller.clone()).to_string(),
                        to: Hex(call.call.address.clone()).to_string(),
                        amount: formatted_value,
                        token_type: 4,
                        block_number: blk.number,
                        block_timestamp: blk.clone()
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
                    transfers.push(base_token_transfer); 
                }
                None =>{}
            }
        }

        for log in trx.receipt().logs(){
            // ERC20 Transfer
            if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log){
                let erc20_transfer: Transfer = Transfer{
                    amount: event.value.to_string(),
                    token_id: event.value.to_string(),
                    token_address: Hex(log.clone().address()).to_string(),
                    chain_id: 1.to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    transaction_hash: Hex(trx.hash.clone()).to_string(),
                    operator: Hex(trx.from.clone()).to_string(),
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    token_type: 0,
                    block_number: blk.number,
                    block_timestamp: blk.clone()
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
            if let Some(event) = abis::erc721::events::Transfer::match_and_decode(log){
                let erc721_transfer: Transfer = Transfer{
                    amount: event.token_id.to_string(),
                    token_address: Hex(log.clone().address()).to_string(),
                    chain_id: 1.to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    transaction_hash: Hex(trx.hash.clone()).to_string(),
                    token_id: event.token_id.to_string(),
                    operator: Hex(trx.from.clone()).to_string(),
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    token_type: 0,  // token type will be detected outside this substream by token tracker substream
                    block_number: blk.number,
                    block_timestamp: blk.clone()
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
            if let Some(event) = abis::erc1155::events::TransferBatch::match_and_decode(log){
                let erc1155_transfer_batch: Erc1155TransferBatch = Erc1155TransferBatch{
                    amounts: event.values.iter().map(|c| c.clone().to_string()).collect(),
                    transaction_hash: Hex(trx.hash.clone()).to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    chain_id: 1.to_string(),
                    token_address: Hex(log.log.clone().address).to_string(),
                    operator: Hex(event.operator).to_string(),
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    token_type:3,
                    block_number: blk.number,
                    block_timestamp: blk.clone()
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64,
                    token_ids: event.ids.iter().map(|id| Hex(id.clone()).0.to_string()).collect(),
                    // log_ordinal: log.ordinal(),
                    ..Default::default()
                };
                erc1155_transfer_batchs.push(erc1155_transfer_batch);
            }
            
            // ERC1155 Batch
            if let Some(event) = abis::erc1155::events::TransferSingle::match_and_decode(log){
                let erc1155_transfer_single: Erc1155TransferSingle = Erc1155TransferSingle{
                    amount: event.value.to_string(),
                    transaction_hash: Hex(trx.hash.clone()).to_string(),
                    log_index: log.block_index(),
                    source: 1,
                    chain_id: 1.to_string(),
                    token_address: Hex(log.log.clone().address).to_string(),
                    operator: Hex(event.operator).to_string(),
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    token_type:3,
                    block_number: blk.number,
                    block_timestamp: blk.clone()
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
    
    }
    Ok(Transfers{ transfers, erc1155_transfer_batchs, erc1155_transfer_singles })
}

#[substreams::handlers::store]
fn store_transfers(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.transfers{
        let key = transfer.clone().transaction_hash;
        // key.push_str(&transfer.log_ordinal.to_string());
        output.set(
            0,
            key,
            &proto::encode(&transfer).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_batch(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.erc1155_transfer_batchs{
        let key = transfer.clone().transaction_hash;
        // key.push_str(&transfer..to_string());
        output.set(
            0, 
            key, 
            &proto::encode(&transfer).unwrap()
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_single(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.erc1155_transfer_singles{
        let key = transfer.clone().transaction_hash;
        // key.push_str(&transfer.log_ordinal.to_string());
        output.set(
            0, 
            key, 
            &proto::encode(&transfer).unwrap()
        );
    }
}
mod abis;
mod pb;

use pb::token_tracker::{Transfers, Transfer, Erc1155TransferBatch, Erc1155TransferSingle};
use substreams::store;
use substreams::{log, Hex, proto};
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    let mut erc1155_transfer_batchs = vec![];
    let mut erc1155_transfer_singles = vec![];
    for log in blk.logs(){
        match &log.receipt.transaction.value {
            Some(value) =>{
                log::info!("Tx Hash {}", Hex(log.receipt.transaction.hash.clone()).to_string());
                let len = 8.min(value.bytes.len());
                let mut value_processed = [0u8; 8];
                value_processed[..len].copy_from_slice(&value.bytes[..len]);
                let base_token_transfer: Transfer = Transfer{
                    amount: u64::from_be_bytes(value_processed[0..8].try_into().unwrap()).to_string(),
                    token_address: String::from("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"),
                    chain_id:1.to_string(),
                    hash:Hex(log.receipt.transaction.hash.clone()).to_string(),
                    from: Hex(log.receipt.transaction.from.clone()).to_string(),
                    to: Hex(log.receipt.transaction.to.clone()).to_string(),
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
                    log_ordinal: log.ordinal(),
                    ..Default::default()
                };
                transfers.push(base_token_transfer); 
            }
            None => {}
        }

        if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log){
            log::info!("Tx Hash {}", Hex(log.receipt.transaction.hash.clone()).to_string());
            let erc20_transfer: Transfer = Transfer{
                amount: event.value.to_string(),
                token_address: Hex(log.log.clone().address).to_string(),
                chain_id: 1.to_string(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
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
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
    
            transfers.push(erc20_transfer);
        }
        if let Some(event) = abis::erc721::events::Transfer::match_and_decode(log){
            log::info!("Tx Hash {}", Hex(log.receipt.transaction.hash.clone()).to_string());
            let erc721_transfer: Transfer = Transfer{
                amount: 1.to_string(),
                chain_id: 1.to_string(),
                token_address: Hex(log.log.clone().address).to_string(),
                token_id: event.token_id.to_string(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                token_type:2,
                block_number: blk.clone().number,
                block_timestamp: blk.clone()
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            transfers.push(erc721_transfer);
        }
        if let Some(event) = abis::erc1155::events::TransferBatch::match_and_decode(log){
            log::info!("Tx Hash {}", Hex(log.receipt.transaction.hash.clone()).to_string());
            let erc1155_transfer_batch: Erc1155TransferBatch = Erc1155TransferBatch{
                amounts: event.values.iter().map(|c| c.clone().to_string()).collect(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                chain_id: 1.to_string(),
                token_address: Hex(log.log.clone().address).to_string(),
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                token_type:3,
                block_number: blk.clone().number,
                block_timestamp: blk.clone()
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
                token_ids: event.ids.iter().map(|id| Hex(id.clone()).0.to_string()).collect(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_batchs.push(erc1155_transfer_batch);
        }
        if let Some(event) = abis::erc1155::events::TransferSingle::match_and_decode(log){
            log::info!("Tx Hash {}", Hex(log.receipt.transaction.hash.clone()).to_string());
            let erc1155_transfer_single: Erc1155TransferSingle = Erc1155TransferSingle{
                amount: event.value.to_string(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                chain_id: 1.to_string(),
                token_address: Hex(log.log.clone().address).to_string(),
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                token_type:3,
                block_number: blk.clone().number,
                block_timestamp: blk.clone()
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds as u64,
                token_id: event.id.to_string(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_singles.push(erc1155_transfer_single);
        }
    }
    Ok(Transfers{ transfers, erc1155_transfer_batchs, erc1155_transfer_singles })
}

#[substreams::handlers::store]
fn store_transfers(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.transfers{
        let mut key = transfer.clone().hash;
        key.push_str(&transfer.log_ordinal.to_string());
        output.set(
            transfer.log_ordinal,
            key,
            &proto::encode(&transfer).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_batch(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.erc1155_transfer_batchs{
        let mut key = transfer.clone().hash;
        key.push_str(&transfer.log_ordinal.to_string());
        output.set(
            transfer.log_ordinal, 
            key, 
            &proto::encode(&transfer).unwrap()
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_single(transfers: Transfers, output: store::StoreSet){
    for transfer in transfers.erc1155_transfer_singles{
        let mut key = transfer.clone().hash;
        key.push_str(&transfer.log_ordinal.to_string());
        output.set(
            transfer.log_ordinal, 
            key, 
            &proto::encode(&transfer).unwrap()
        );
    }
}
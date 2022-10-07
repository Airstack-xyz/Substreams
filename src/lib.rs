mod abis;
mod pb;
mod rpc;

use pb::token_tracker::{AirTransfers, AirTransfer, AirErc1155TransferBatch, AirErc1155TransferSingle};
use substreams::store;
use substreams::{log, Hex, proto};
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<AirTransfers, Error> {
    let mut air_transfers = vec![];
    let mut air_transfer_batchs = vec![];
    let mut air_transfer_singles = vec![];
    for log in blk.logs(){
        if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log){
            log::info!("transfer event: {}", Hex(&event.to));
            // log::info!("ERC20 token: {}, {}, {}", Hex(log.log.clone().address), log.receipt.transaction.hash.clone().to_string(), log.ordinal());

            let air_transfer: AirTransfer = AirTransfer{
                amount: event.value.to_string(),
                token_id: Hex(log.log.clone().address).to_string(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                token_type: 0,
                block_number: blk.clone().number,
                block_timestamp: blk.clone()
                .header
                .as_ref()
                .unwrap()
                .timestamp
                .as_ref()
                .unwrap()
                .seconds
                .to_string(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
    
            air_transfers.push(air_transfer);
        }
        if let Some(event) = abis::erc721::events::Transfer::match_and_decode(log){
            log::info!("transfer event: {}", Hex(&event.to));
            // log::info!("ERC721 token: {}, {}", log.receipt.transaction.hash.clone().to_string(), log.ordinal());
            let air_transfer: AirTransfer = AirTransfer{
                amount: 1.to_string(),
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
                .seconds
                .to_string(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            air_transfers.push(air_transfer);
        }
        if let Some(event) = abis::erc1155::events::TransferBatch::match_and_decode(log){
            log::info!("transfer event: {}", Hex(&event.to));
            // log::info!("transfer values: {}", &event.values);
            let air_transfer_batch: AirErc1155TransferBatch = AirErc1155TransferBatch{
                amounts: event.values.iter().map(|c| c.clone().to_string()).collect(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
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
                .seconds
                .to_string(),
                token_ids: event.ids.iter().map(|id| Hex(id.clone()).0.to_string()).collect(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            air_transfer_batchs.push(air_transfer_batch);
        }
        if let Some(event) = abis::erc1155::events::TransferSingle::match_and_decode(log){
            log::info!("transfer event: {}", Hex(&event.to));
            let air_transfer_single: AirErc1155TransferSingle = AirErc1155TransferSingle{
                amount: event.value.to_string(),
                hash: Hex(log.receipt.transaction.hash.clone()).to_string(),
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
                .seconds
                .to_string(),
                token_id: event.id.to_string(),
                log_ordinal: log.ordinal(),
                ..Default::default()
            };
            air_transfer_singles.push(air_transfer_single);
        }
    }
    Ok(AirTransfers{ air_transfers, air_transfer_batchs, air_transfer_singles })
}

#[substreams::handlers::store]
fn store_transfers(transfers: AirTransfers, output: store::StoreSet){
    for transfer in transfers.air_transfers{
        output.set(
            transfer.log_ordinal,
            &transfer.hash,
            &proto::encode(&transfer).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_batch(transfers: AirTransfers, output: store::StoreSet){
    for transfer in transfers.air_transfer_batchs{
        output.set(
            transfer.log_ordinal, 
            &transfer.hash, 
            &proto::encode(&transfer).unwrap()
        );
    }
}

#[substreams::handlers::store]
fn store_erc1155_single(transfers: AirTransfers, output: store::StoreSet){
    for transfer in transfers.air_transfer_singles{
        output.set(
            transfer.log_ordinal, 
            &transfer.hash, 
            &proto::encode(&transfer).unwrap()
        );
    }
}
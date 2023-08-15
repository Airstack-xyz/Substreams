mod abis;
mod helpers;
mod pb;

use hex_literal::hex;
use pb::lens_follower_tracker::{FollowNftDeployed, FollowNftDeployedMessage};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

// Lens LPP contract 
const LENS_LPP_CONTRACT: [u8; 20] = hex!("Db46d1Dc155634FbC732f92E853b10B288AD5a1d");


#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<FollowNftDeployedMessage, Error> {
    let mut followNftDeployedEvent: Vec<FollowNftDeployed> = vec![];
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        log::info!("Tx Hash {}", tx_hash);
        // if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log) {

        // }

        // // ERC20 Transfer
        // if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log) {
        //     let erc20_transfer: Transfer = Transfer {
        //         amount: event.value.to_string(),
        //         token_id: event.value.to_string(),
        //         token_address: helpers::utils::format_with_0x(
        //             Hex(log.clone().address()).to_string(),
        //         ),
        //         chain_id: 1.to_string(),
        //         log_index: log.block_index(),
        //         transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
        //         operator: helpers::utils::format_with_0x(
        //             Hex(log.receipt.transaction.from.clone()).to_string(),
        //         ),
        //         from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
        //         to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
        //         block_number: blk.number,
        //         block_timestamp: blk
        //             .clone()
        //             .header
        //             .as_ref()
        //             .unwrap()
        //             .timestamp
        //             .as_ref()
        //             .unwrap()
        //             .seconds as u64,
        //         // log_ordinal: log.ordinal(),
        
        //         ..Default::default()
        //     };
        //     transfers.push(erc20_transfer);
        // }
       
    }
    Ok(FollowNftDeployedMessage { follow_nft_deployed_events: followNftDeployedEvent } 
      )
}

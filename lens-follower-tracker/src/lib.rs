mod abis;
mod pb;
mod helpers;

use helpers::constants::{ZERO_ADDRESS, FOLLOW, UNFOLLOW, EMPTY_STRING};
use pb::lens::events::{FollowNftDeployed, FollowEvents, FollowNftTransferred};
use substreams::errors::Error;
use substreams::Hex;
use serde::Deserialize;
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use helpers::utils::{format_with_0x, is_address_valid};
use anyhow::anyhow;


substreams_ethereum::init!();


#[derive(Deserialize)]
struct InputFilters {
    lpp_contract_address: Option<String>,
    proxy_action_contract_address: Option<String>,
}

/// Extracts lens events from the contract
#[substreams::handlers::map]
fn map_lens_follower_events(params: String, blk: eth::Block) -> Result<FollowEvents, Vec<Error>> {
    let filters = parse_filters_from_params(params)?;
    let lpp_contract_address = filters.lpp_contract_address.expect("lpp_contract_address is already verified");
    let proxy_action_contract_address = filters.proxy_action_contract_address.expect("proxy_action_contract_address already verified");
    let mut follow_nft_deployed_events = vec![];
    let mut follow_nft_transferred_events: Vec<FollowNftTransferred> = vec![];

    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        let addres : String = Hex(log.address().clone()).to_string();
        if !lpp_contract_address.contains(&addres) {
           continue;
        }
        // for FollowNftDeployed event
        if let Some(event) = abis::lens_events::events::FollowNftDeployed::match_and_decode(log) {
            let follow_nft_deployed_event : FollowNftDeployed = FollowNftDeployed {
                follow_profile_id: event.profile_id.to_string(),
                follow_token_address: format_with_0x(Hex(event.follow_nft).to_string()),
                activity_timestamp: event.timestamp.to_u64(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index().to_string(),
                block_number:  blk.number,
            };
            follow_nft_deployed_events.push(follow_nft_deployed_event);
            continue;
        } 

        // for Followed event
        if let Some(event) = abis::lens_events::events::FollowNftTransferred::match_and_decode(log) {
            let from = format_with_0x(Hex(event.from).to_string());
            let to = format_with_0x(Hex(event.to).to_string());
            let mut follow_nft_transferred_event : FollowNftTransferred = FollowNftTransferred {
                follow_profile_id: event.profile_id.to_string(),
                follow_token_id: event.follow_nft_id.to_string(),
                activity_timestamp: event.timestamp.to_u64(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index().to_string(),
                block_number:  blk.number,
                ..Default::default() 
            };
            
            // scenario 1: follow event [from:0x to: follower address]
            // example tx hash: 0xdea7fee50dd48d3a6b03d5ed39e41a4df68b311a58d2f38b47e88d721c85a979
            if from == ZERO_ADDRESS && to != proxy_action_contract_address {
                follow_nft_transferred_event.follower_address = to.clone();
                follow_nft_transferred_event.follow_type =  FOLLOW.to_string();    
            }

            // scenario 2: follow for event [from:0x..782 [proxy] to: follower address]
            // example tx hash: 0xbc3c64e31c901f0560fb55e1ff8635d91b5a7ab8f5d5b5d50b6053a3907b1f10
            if from == proxy_action_contract_address && to != ZERO_ADDRESS {
                follow_nft_transferred_event.follower_address = to.clone();
                follow_nft_transferred_event.follow_type =  FOLLOW.to_string();  
            }

            // scenario 3: burn event [from:follower to: 0x]
            // example tx hash: 0x68d80b53990cc9b3d7e48d6c11295f2ede7cbff8ae6a05b7c265bcccf9d994bc
            if from != proxy_action_contract_address && to == ZERO_ADDRESS {
                follow_nft_transferred_event.follower_address = from.clone();
                follow_nft_transferred_event.follow_type =  UNFOLLOW.to_string();               
            }

            // scenario 4: transfer between addresses  -> should unfollow the from address
            // example tx hash: 0xbac904222e15b792acd053f86a8834ebbb4414bedbf4051b0297c378bde73f38
            if from != ZERO_ADDRESS && from != proxy_action_contract_address 
                && to != ZERO_ADDRESS && to != proxy_action_contract_address {
                    follow_nft_transferred_event.follower_address = from.clone();
                    follow_nft_transferred_event.follow_type =  UNFOLLOW.to_string();                    
            }
            // finally push the data to vector
            if follow_nft_transferred_event.follow_type != EMPTY_STRING {
                follow_nft_transferred_events.push(follow_nft_transferred_event); 
            }
        }
    }

    Ok(FollowEvents {
        follow_nft_deployed_events,
        follow_nft_transferred_events
    })
}

fn parse_filters_from_params(params: String) -> Result<InputFilters, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params);
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!("Unpexcted error while parsing parameters")]));
    }

    let filters = parsed_result.unwrap();
    verify_filters(&filters)?;

    Ok(filters)
}

fn verify_filters(params: &InputFilters) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    let lpp_contract_address = params.lpp_contract_address.as_ref().unwrap();
    if !is_address_valid(lpp_contract_address) {
        errors.push(anyhow!("'lpp_contract_address' address ({}) is not valid", lpp_contract_address));
    }
    
    let proxy_action_contract_address = params.proxy_action_contract_address.as_ref().unwrap();
    if !is_address_valid(proxy_action_contract_address) {
        errors.push(anyhow!("'proxy_action_contract_address' address ({}) is not valid", proxy_action_contract_address));
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(())
}
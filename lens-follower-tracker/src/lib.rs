mod abis;
mod helpers;
mod pb;

use anyhow::anyhow;
use helpers::constants::{EMPTY_STRING, ZERO_ADDRESS};
use helpers::utils::{format_with_0x, is_address_valid};
use pb::lens::events::{FollowEvents, FollowNftDeployed, FollowNftTransferred, ProfileTransferred};
use serde::Deserialize;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

substreams_ethereum::init!();

#[derive(Deserialize)]
struct InputFilters {
    lpp_contract_address: Option<String>,
}

/// Extracts lens events from the contract
#[substreams::handlers::map]
fn map_lens_follower_events(params: String, blk: eth::Block) -> Result<FollowEvents, Vec<Error>> {
    let filters: InputFilters = parse_filters_from_params(params)?;
    let lpp_contract_address = filters
        .lpp_contract_address
        .expect("lpp_contract_address is already verified");
    let mut follow_nft_deployed_events: Vec<FollowNftDeployed> = vec![];
    let mut follow_nft_transferred_events: Vec<FollowNftTransferred> = vec![];
    let mut profile_transferred_events: Vec<ProfileTransferred> = vec![];

    
    // below block will process the follow related events
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        let addres: String = Hex(log.address().clone()).to_string();
        if !lpp_contract_address.contains(&addres) {
            continue;
        }

        // for lens profile transfer scenarios
        if let Some(event) = abis::erc721_events::events::Transfer::match_and_decode(log) {
            let from = helpers::utils::format_with_0x(Hex(event.from).to_string());
            let to = helpers::utils::format_with_0x(Hex(event.to).to_string());
            if from == ZERO_ADDRESS && to != ZERO_ADDRESS {
                continue;
            }
            let profile_transferred_event: ProfileTransferred = ProfileTransferred {
                from,
                to,
                profile_token_id: event.token_id.to_string(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index().to_string(),
                block_number: blk.number,
            };
            profile_transferred_events.push(profile_transferred_event);
            continue;
        }

        // for FollowNftDeployed event
        if let Some(event) = abis::lens_events::events::FollowNftDeployed::match_and_decode(log) {
            let follow_nft_deployed_event: FollowNftDeployed = FollowNftDeployed {
                follow_profile_id: event.profile_id.to_string(),
                follow_token_address: format_with_0x(Hex(event.follow_nft).to_string()),
                activity_timestamp: event.timestamp.to_u64(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index().to_string(),
                block_number: blk.number,
            };
            follow_nft_deployed_events.push(follow_nft_deployed_event);
            continue;
        }

        // for Followed event
        // scenario 1: follow event [from:0x0  to: follower address]
        // example tx hash: 0xdea7fee50dd48d3a6b03d5ed39e41a4df68b311a58d2f38b47e88d721c85a979

        // scenario 2: follow for event [from:0x0  to: proxy address] [from: proxy to: follower address]
        // example tx hash: 0xbc3c64e31c901f0560fb55e1ff8635d91b5a7ab8f5d5b5d50b6053a3907b1f10

        // scenario 3: burn event [from:follower address to: 0x0 ]
        // example tx hash: 0x68d80b53990cc9b3d7e48d6c11295f2ede7cbff8ae6a05b7c265bcccf9d994bc

        // scenario 4: transfer between addresses  -> should unfollow the from address
        // example tx hash: 0xbac904222e15b792acd053f86a8834ebbb4414bedbf4051b0297c378bde73f38

        // scenario 5: follow event from [from:0x0 to: proxy address] in a separate tx hash
        // and followNFTTransferred event from [from:proxy address to: follower address] in a separate tx hash
        // example: https://polygonscan.com/nft/0x540ca34e166a134dd649385679f3fe4447e0ae33/42908

        if let Some(event) = abis::lens_events::events::FollowNftTransferred::match_and_decode(log)
        {
            let from = format_with_0x(Hex(event.from).to_string());

            let follow_nft_transferred_event: FollowNftTransferred = FollowNftTransferred {
                follow_profile_id: event.profile_id.to_string(),
                follow_token_id: event.follow_nft_id.to_string(),
                activity_timestamp: event.timestamp.to_u64(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index().to_string(),
                block_number: blk.number,
                follower_address: from,
                follow_type: EMPTY_STRING.to_string(),
                ..Default::default()
            };

            follow_nft_transferred_events.push(follow_nft_transferred_event);
        }
    }

    Ok(FollowEvents {
        follow_nft_deployed_events,
        follow_nft_transferred_events,
        profile_transferred_events,
    })
}

fn parse_filters_from_params(
    params: String,
) -> Result<InputFilters, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params);
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!(
            "Unpexcted error while parsing parameters"
        )]));
    }

    let filters = parsed_result.unwrap();
    verify_filters(&filters)?;

    Ok(filters)
}

fn verify_filters(params: &InputFilters) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    let lpp_contract_address = params.lpp_contract_address.as_ref().unwrap();
    if !is_address_valid(lpp_contract_address) {
        errors.push(anyhow!(
            "'lpp_contract_address' address ({}) is not valid",
            lpp_contract_address
        ));
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(())
}

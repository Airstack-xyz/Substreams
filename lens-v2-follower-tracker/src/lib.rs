mod abis;
mod helpers;
mod pb;

use anyhow::anyhow;
use helpers::utils::{is_address_valid, format_with_0x};
use pb::lens::events::{Follow, FollowEvents, Unfollow};
use serde::Deserialize;
use substreams::errors::Error;
use substreams::{Hex, log};
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
    let mut follow_events: Vec<Follow> = vec![];
    let mut unfollow_events: Vec<Unfollow> = vec![];

    log::info!("block {}", blk.number);
    // below block will process the follow related events
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        let address: String = format_with_0x(Hex(log.address().clone()).to_string());

        if !lpp_contract_address.to_lowercase().contains(&address) {
            continue;
        }
        if let Some(event) = abis::lens_events::events::Followed::match_and_decode(log) {
            let follow_event: Follow = Follow {
                follower_profile_id: event.follower_profile_id.to_string(),
                id_of_profile_followed: event.id_of_profile_followed.to_string(),
                follow_token_id_assigned: event.follow_token_id_assigned.to_string(),
                follow_module_data: format_with_0x(Hex(event.follow_module_data).to_string()),
                process_follow_module_return_data: format_with_0x(Hex(event.process_follow_module_return_data).to_string())
                    .to_string(),
                transaction_executor:format_with_0x(Hex(event.transaction_executor).to_string()),
                timestamp: event.timestamp.to_u64(),
                transaction_hash: format_with_0x(tx_hash),
                log_index: log.block_index().to_string(),
                block_number: blk.number,
            };
            follow_events.push(follow_event);
            continue;
        }

        if let Some(event) = abis::lens_events::events::Unfollowed::match_and_decode(log) {
            let unfollow_event: Unfollow = Unfollow {
                unfollower_profile_id: event.unfollower_profile_id.to_string(),
                id_of_profile_unfollowed: event.id_of_profile_unfollowed.to_string(),
                transaction_executor: format_with_0x(Hex(event.transaction_executor).to_string()),
                timestamp: event.timestamp.to_u64(),
                transaction_hash: format_with_0x(tx_hash),
                log_index: log.block_index().to_string(),
                block_number: blk.number,
            };
            unfollow_events.push(unfollow_event);
            continue;
        }
    }

    Ok(FollowEvents {
        follow_events,
        unfollow_events,
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

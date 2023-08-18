mod abis;
mod pb;
mod helpers;

use pb::lens::events::{FollowNftDeployed, FollowEvents, FollowNftTransferred};
use substreams::errors::Error;
use substreams::{Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};
use helpers::utils::format_with_0x;


// Lens LPP contract 
// const LENS_LPP_CONTRACT : &str = "db46d1dc155634fbc732f92e853b10b288ad5a1d";
const ZERO_ADDRESS : &str = "0x0000000000000000000000000000000000000000";
const LENS_PROXY_ACTION_ADDRESS : &str = "0xdd3f6c22ecc68007cc9f76da18984995da4b7b82";
const FOLLOW : &str = "follow";
const UNFOLLOW : &str = "unfollow";

substreams_ethereum::init!();

/// Extracts lens events from the contract
#[substreams::handlers::map]
fn map_lens_follower_events(contract_address: String, blk: eth::Block) -> Result<FollowEvents, Error> {
    let mut follow_nft_deployed_events = vec![];
    let mut follow_nft_transferred_events: Vec<FollowNftTransferred> = vec![];

    for log in blk.logs() {
        let addres : String = Hex(log.address().clone()).to_string();
        if !contract_address.contains(&addres) {
           continue;
        }
        // for FollowNftDeployed event
        if let Some(event) = abis::lens_events::events::FollowNftDeployed::match_and_decode(log) {
            let follow_nft_deployed_event : FollowNftDeployed = FollowNftDeployed {
                follow_profile_id: event.profile_id.to_string(),
                follow_token_address: helpers::utils::format_with_0x(Hex(event.follow_nft).to_string())
            };
            follow_nft_deployed_events.push(follow_nft_deployed_event)
        } 

        // for Followed event
        if let Some(event) = abis::lens_events::events::FollowNftTransferred::match_and_decode(log) {
            
            let from = format_with_0x(Hex(event.from).to_string());
            let to = format_with_0x(Hex(event.to).to_string());
            
            // scenario 1: follow event [from:0x to: follower address]
            if from == ZERO_ADDRESS && to != LENS_PROXY_ACTION_ADDRESS {
                let follow_nft_transferred_event : FollowNftTransferred = FollowNftTransferred {
                    follow_profile_id: event.profile_id.to_string(),
                    follow_token_id: event.follow_nft_id.to_string(),
                    follower_address: to.clone(),
                    follow_type: FOLLOW.to_string(),
                };
                follow_nft_transferred_events.push(follow_nft_transferred_event) 
            }

            // scenario 2: follow for event [from:0x..782 [proxy] to: follower address]
            if from == LENS_PROXY_ACTION_ADDRESS && to != ZERO_ADDRESS {
                let follow_nft_transferred_event : FollowNftTransferred = FollowNftTransferred {
                    follow_profile_id: event.profile_id.to_string(),
                    follow_token_id: event.follow_nft_id.to_string(),
                    follower_address: to.clone(),
                    follow_type: FOLLOW.to_string(),
                };
                follow_nft_transferred_events.push(follow_nft_transferred_event) 
            }

            // scenario 3: burn event [from:follower to: 0x]
            if from != LENS_PROXY_ACTION_ADDRESS && to == ZERO_ADDRESS {
                let follow_nft_transferred_event : FollowNftTransferred = FollowNftTransferred {
                    follow_profile_id: event.profile_id.to_string(),
                    follow_token_id: event.follow_nft_id.to_string(),
                    follower_address: from.clone(),
                    follow_type: UNFOLLOW.to_string(),
                };
                follow_nft_transferred_events.push(follow_nft_transferred_event) 
            }

            // scenario 4: transfer between addresses  -> should unfollow the from address
            if from != ZERO_ADDRESS && from != LENS_PROXY_ACTION_ADDRESS 
                && to != ZERO_ADDRESS && to != LENS_PROXY_ACTION_ADDRESS {
                    let follow_nft_transferred_event : FollowNftTransferred = FollowNftTransferred {
                        follow_profile_id: event.profile_id.to_string(),
                        follow_token_id: event.follow_nft_id.to_string(),
                        follower_address: from.clone(),
                        follow_type: UNFOLLOW.to_string(),
                    };
                    follow_nft_transferred_events.push(follow_nft_transferred_event) 
            }
        }
    }

    Ok(FollowEvents {
        follow_nft_deployed_events,
        follow_nft_transferred_events
    })
}

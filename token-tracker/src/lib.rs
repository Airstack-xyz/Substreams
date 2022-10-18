mod abi;
pub mod helper;
mod pb;
use pb::token_tracker;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts transfer events from the contract

#[substreams::handlers::map]
fn block_to_tokens(blk: eth::Block) -> Result<token_tracker::Tokens, substreams::errors::Error> {
    let mut tokens = token_tracker::Tokens { tokens: vec![] };

    
    let block_number = blk.clone().number.to_string();
    let block_timestamp = blk.clone().header.unwrap().timestamp.unwrap().seconds.to_string();
    for call_view in blk.calls() {
        let tx_hash = Hex(call_view.transaction.clone().hash).to_string();
        let from = Hex(call_view.transaction.from.clone()).to_string();

        let call = call_view.call;
        if call.call_type == eth::CallType::Create as i32 {
            let address = Hex(call.address.clone()).to_string();

            log::info!("address {}", address);

            let token = helper::get_token(
                address,
                block_number.clone(),
                block_timestamp.clone(),
                tx_hash,
                from
            );
            if  token.is_some(){
                tokens.tokens.push(token.unwrap());
            }
        }
    }
    Ok(tokens)
}

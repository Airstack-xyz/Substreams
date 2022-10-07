// use bigdecimal::BigDecimal;
// use num_bigint::BigInt;
// use substreams::log;
// use substreams_ethereum::pb::eth as ethpb;

// pub fn get_wallet_balance(wallet_address: &String) -> BigInt{
//     let rpc_calls: ethpb::rpc::RpcCalls = create_wallet_balance_call(&hex::decode(wallet_address).unwrap());

//     let rpc_responses_unmarshalled: ethpb::rpc::RpcResponses = substreams_ethereum::rpc::eth_call(&rpc_calls);
//     let responses = rpc_responses_unmarshalled.responses;

//     return BigInt::from_signed_bytes_be(responses[0].raw.as_ref());
// }

// fn create_wallet_balance_call(addr: &Vec<u8>) -> ethpb::rpc::RpcCalls {
//     let wallet_balance_data = hex::decode("18160ddd").unwrap();

//     return ethpb::rpc::RpcCalls {
//         calls: vec![ethpb::rpc::RpcCall {
//             to_addr: Vec::from(addr.clone()),
//             data: wallet_balance_data
//         }]
//     }
// }
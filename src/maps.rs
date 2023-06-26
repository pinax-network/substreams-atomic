use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::eosio_nfts::*;

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.name != "logtransfer" { continue; }
            match abi::Logtransfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let converted_asset_ids: Vec<u64> = data.asset_ids
                        .iter()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
                    response.push(TransferEvent {
                        trx_id: trx.id.clone(),
                        
                        collection_name: data.collection_name,
                        from: data.from,
                        to: data.to,
                        asset_ids: converted_asset_ids,
                        memo: data.memo,
                    });
                }
                Err(_) => panic!("Failed to decode atomicassets::transfer"),
           }
        }
    }
    Ok(TransferEvents { items: response })
}

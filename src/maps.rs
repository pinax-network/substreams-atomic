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
            if trace.receiver != "atomicassets" {continue}
            if action_trace.account != "atomicassets" && action_trace.name != "transfer" { continue; }
            match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let converted_asset_ids: Vec<u64> = data.asset_ids
                        .iter()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
                    response.push(TransferEvent {
                        trx_id: trx.id.clone(),

                        // contract & scope
                        // contract: action_trace.account.clone(),
                        // action: action_trace.name.clone(),
                        // payload
                        from: data.from,
                        to: data.to,
                        asset_ids: converted_asset_ids,
                        memo: data.memo,
                    });
                }
                Err(_) => continue,
           }
        }
    }
    Ok(TransferEvents { items: response })
}

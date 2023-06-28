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
                        //memo: data.memo,
                    });
                }
                Err(_) => panic!("Failed to decode atomicassets::logtransfer"),
           }
        }
    }
    Ok(TransferEvents { items: response })
}

#[substreams::handlers::map]
fn map_schemas(block: Block) -> Result<SchemaEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.name != "createschema" { continue; }
            match abi::Createschema::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let mut formats = vec![];
                    for f in &data.schema_format {
                        formats.push(Format {
                            name: f.name.clone(),
                            dtype: f.r#type.clone(),
                        });
                    }
                    response.push(SchemaEvent {
                        // trace information
                        trx_id: trx.id.clone(),

                        // payload
                        authorized_creator: data.authorized_creator,
                        collection_name: data.collection_name,
                        schema_name: data.schema_name,
                        schema_format : formats,
                    });
                }
                Err(_) => {continue} //panic!("Failed to decode atomicassets::createschema"),
           }
        }
    }
    Ok(SchemaEvents { items: response })
}

#[substreams::handlers::map]
fn map_collections(block: Block) -> Result<Collections, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "collections" { continue; }
            let contract = db_op.scope.clone();


            // Looks like old and new data is always none
            let old_data = abi::CollectionsS::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::CollectionsS::try_from(db_op.new_data_json.as_str()).ok();

            // comment below line if you want to see any transaction appearing
            if old_data.is_none() && new_data.is_none() { continue; } // no data

            let action = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };
            
            items.push(Collection {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                action: action.to_string(),

                // payload
                //collection_name: new_name.expect("REASON").to_string(),
                
            });
        }
    }
    Ok(Collections { items })
}
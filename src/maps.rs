use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicassets::*;

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

            let old_data = abi::CollectionsS::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::CollectionsS::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no data
            
            let action = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let collection_name = match &new_data {
                Some(data) => data.collection_name.clone(),
                None => "No new data".to_string(),
            };

            let author = match &new_data {
                Some(data) => data.author.clone(),
                None => "No new data".to_string(),
            };

            let allow_notify = match &new_data {
                Some(data) => data.allow_notify.clone(),
                None => false,
            };

            let old_authorized_accounts = match &old_data {
                Some(data) => Some(Vec::<String>::from(data.authorized_accounts.clone())),
                None => None,
            };

            let new_authorized_accounts = match &new_data {
                Some(data) => Some(Vec::<String>::from(data.authorized_accounts.clone())),
                None => None,
            };

            let authorized_accounts = match new_authorized_accounts.is_some() {
                true => new_authorized_accounts.unwrap(),
                false => [].to_vec(),
            };

            //let authorized_accounts_delta = match old_authorized_accounts.is_some() {
            //    true => authorized_accounts - old_authorized_accounts.unwrap(),
            //    false => authorized_accounts, 
            //};

            let notify_accounts = match &new_data {
                Some(data) => data.notify_accounts.clone(),
                None => [].to_vec(),
            };

            let market_fee = match &new_data {
                Some(data) => data.market_fee.clone(),
                None => 0.0,
            };

            let serialized_data = match &new_data {
                Some(data) => data.serialized_data.clone(),
                None => [].to_vec(),
            };

            items.push(Collection {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                action: action.to_string(),

                // payload
                collection_name: collection_name,
                author: author,
                allow_notify: allow_notify,
                authorized_accounts: authorized_accounts,
                notify_accounts: notify_accounts,
                market_fee: market_fee,
                //serialized_data: Vec::<u32>::from(serialized_data),

                //extra
                //authorized_accounts_delta: authorized_accounts_delta,
            });
        }
    }
    Ok(Collections { items })
}

#[substreams::handlers::map]
fn map_templates(block: Block) -> Result<Templates, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "templates" { continue; }
            let contract = db_op.scope.clone();

            let new_data = match abi::TemplatesS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => panic!("new data not decoded: {}", error),
            };
            //let old_data = abi::TemplatesS::try_from(db_op.old_data_json.as_str()).ok();
            //let new_data = abi::TemplatesS::try_from(db_op.new_data_json.as_str()).ok();
            //if old_data.is_none() && new_data.is_none() { continue; } // no data
            
            let action = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            items.push(Template {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                action: action.to_string(),

                // data payload
                template_id: new_data.template_id.clone(),
                schema_name: new_data.schema_name.clone(),
                transferable: new_data.transferable.clone(),
                burnable: new_data.burnable.clone(),
                max_supply: new_data.max_supply.clone(),
                issued_supply: new_data.issued_supply.clone(),
            });
        }
    }
    Ok(Templates { items })
}
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
                    // filtering only atomicmarket related transfers
                    if data.from != "atomicmarket" && data.to != "atomicmarket" {continue}
                    response.push(TransferEvent {
                        trx_id: trx.id.clone(),
                        timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                        collection_name: data.collection_name,
                        from: data.from,
                        to: data.to,
                        asset_ids: data.asset_ids,
                        memo: data.memo,
                    });
                }
                Err(_) => panic!("Failed to decode atomicassets::logtransfer"),
           }
        }
    }
    Ok(TransferEvents { items: response })
}

#[substreams::handlers::map]
fn map_schema_events(block: Block) -> Result<SchemaEvents, Error> {
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
                        timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

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

            let db_operation = match db_op.operation {
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::CollectionsS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::CollectionsS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }

            items.push(Collection {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,
                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

                // db operation 
                db_operation: db_operation.to_string(),

                // payload
                collection_name: data.collection_name.clone(),
                author: data.author.clone(),
                allow_notify: data.allow_notify.clone(),
                authorized_accounts: data.authorized_accounts.clone(),
                notify_accounts: data.notify_accounts.clone(),
                market_fee: data.market_fee.clone(),

                // Takes too much screen space when printed so commented for now
                //serialized_data: Vec::<u32>::from(data.serialized_data),
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

            
            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::TemplatesS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::TemplatesS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }

            items.push(Template {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,
                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                template_id: data.template_id.clone(),
                schema_name: data.schema_name.clone(),
                transferable: data.transferable.clone(),
                burnable: data.burnable.clone(),
                max_supply: data.max_supply.clone(),
                issued_supply: data.issued_supply.clone(),
                collection_name: db_op.scope.clone(),
            });
        }
    }
    Ok(Templates { items })
}

#[substreams::handlers::map]
fn map_schemas(block: Block) -> Result<Schemas, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "schemas" { continue; }

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::SchemasS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::SchemasS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }

            let mut format = vec![];
            for f in &data.format {
                format.push(Format {
                    name: f.name.clone(),
                    dtype: f.r#type.clone(),
                });
            }

            items.push(Schema {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,
                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                collection_name: db_op.scope.clone(),
                schema_name: data.schema_name.clone(),
                format: format,
            });
        }
    }
    Ok(Schemas { items })
}

#[substreams::handlers::map]
fn map_assets(block: Block) -> Result<Assets, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "assets" { continue; }

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::AssetsS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::AssetsS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }

            items.push(Asset {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,
                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

                // db operation 
                db_operation: db_operation.to_string(),
                scope: db_op.scope.clone(),

                // data payload
                asset_id: data.asset_id.clone(),
                collection_name: data.collection_name.clone(),
                schema_name: data.schema_name.clone(),
                template_id: data.template_id.clone(),
                ram_payer: data.ram_payer.clone(),
            });
        }
    }
    Ok(Assets { items })
}

// Not tested yet
#[substreams::handlers::map]
fn map_balances(block: Block) -> Result<Balances, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "balances" { continue; }

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::BalancesS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::BalancesS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }
            
            items.push(Balance {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                owner: data.owner.clone(),
                quantities: data.quantities.clone(),
            });
        }
    }
    Ok(Balances { items })
}

#[substreams::handlers::map]
fn map_offers(block: Block) -> Result<Offers, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "offers" { continue; }

            
            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let data;
            if db_operation == "Remove" {
                data = match abi::OffersS::try_from(db_op.old_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("old data not decoded: {}", error);
                        continue;
                    }
                };
            }
            else {
                data = match abi::OffersS::try_from(db_op.new_data_json.as_str()){
                    Ok(data) => data,
                    Err(error) => {
                        substreams::log::debug!("new data not decoded: {}", error);
                        continue;
                    }
                };
            }

            items.push(Offer {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,
                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                offer_id: data.offer_id.clone(),
                offer_sender: data.sender.clone(),
                offer_recipient: data.recipient.clone(),
                sender_asset_ids: data.sender_asset_ids.clone(),
                recipient_asset_ids: data.recipient_asset_ids.clone(),
                memo: data.memo.clone(),
                ram_payer: data.ram_payer.clone(),
            });
        }
    }
    Ok(Offers { items })
}

/* Not used for now
#[substreams::handlers::map]
fn map_collection_events(block: Block) -> Result<CollectionEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.name != "setcoldata" { continue; }
            match abi::Setcoldata::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let mut attributes = vec![];
                    for a in &data.data {
                        attributes.push(Attribute {
                            key: a.key,
                            value: a.value.to_string(),
                        });
                    }
                    response.push(CollectionEvent {
                        // trace information
                        trx_id: trx.id.clone(),

                        // payload
                        collection_name: data.collection_name,
                        data: attributes,
                    });
                }
                Err(_) => panic!("Failed to decode atomicassets::setcoldata"),//{continue} 
           }
        }
    }
    Ok(CollectionEvents { items: response })
}
*/
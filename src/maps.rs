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
                    response.push(TransferEvent {
                        trx_id: trx.id.clone(),
                        collection_name: data.collection_name,
                        from: data.from,
                        to: data.to,
                        asset_ids: data.asset_ids,
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

            let new_data = match abi::CollectionsS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
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
                db_operation: db_operation.to_string(),

                // payload
                collection_name: new_data.collection_name.clone(),
                author: new_data.author.clone(),
                allow_notify: new_data.allow_notify.clone(),
                authorized_accounts: new_data.authorized_accounts.clone(),
                notify_accounts: new_data.notify_accounts.clone(),
                market_fee: new_data.market_fee.clone(),

                // Takes too much screen space when printed so commented for now
                //serialized_data: Vec::<u32>::from(new_data.serialized_data),
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

            let new_data = match abi::TemplatesS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
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
                db_operation: db_operation.to_string(),

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

#[substreams::handlers::map]
fn map_schemas(block: Block) -> Result<Schemas, Error> {
    let mut items = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "schemas" { continue; }

            let new_data = match abi::SchemasS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            let mut format = vec![];
            for f in &new_data.format {
                format.push(Format {
                    name: f.name.clone(),
                    dtype: f.r#type.clone(),
                });
            }

            items.push(Schema {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                schema_name: new_data.schema_name.clone(),
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

            let new_data = match abi::AssetsS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            items.push(Asset {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                asset_id: new_data.asset_id.clone(),
                collection_name: new_data.collection_name.clone(),
                schema_name: new_data.schema_name.clone(),
                template_id: new_data.template_id.clone(),
                ram_payer: new_data.ram_payer.clone(),
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

            let new_data = match abi::BalancesS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            items.push(Balance {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                owner: new_data.owner.clone(),
                quantities: new_data.quantities.clone(),
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

            let new_data = match abi::OffersS::try_from(db_op.new_data_json.as_str()){
                Ok(data) => data,
                Err(error) => {
                    substreams::log::debug!("new data not decoded: {}", error);
                    continue;
                }
            };

            let db_operation = match db_op.operation{
                0 => "Unknown",
                1 => "Insert",
                2 => "Update",
                3 => "Remove",
                _ => "Error",
            };

            items.push(Offer {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // db operation 
                db_operation: db_operation.to_string(),

                // data payload
                offer_id: new_data.offer_id.clone(),
                offer_sender: new_data.sender.clone(),
                offer_recipient: new_data.recipient.clone(),
                sender_asset_ids: new_data.sender_asset_ids.clone(),
                recipient_asset_ids: new_data.recipient_asset_ids.clone(),
                memo: new_data.memo.clone(),
                ram_payer: new_data.ram_payer.clone(),
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
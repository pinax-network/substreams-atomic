// Generated by antelope-abi2rs 0.4.1 - eosio::abi/1.1

use serde::{Deserialize, Deserializer, Serialize};

type Asset = String;
type Name = String;
type Bool = bool;
type Bytes = String;
type Symbol = String;
type Int16 = i16;
type Int32 = i32;
type Int64 = i64;
type Uint8 = u8;
type Uint16 = u16;
type Uint32 = u32;
type Uint64 = u64;
type Float32 = String;
type Float64 = f64;

fn str_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrI64<'a> {
        Str(&'a str),
        I64(i64),
    }

    Ok(match StrOrI64::deserialize(deserializer)? {
        StrOrI64::Str(v) => v
            .parse::<i64>()
            .map_err(|_| serde::de::Error::custom("failed to parse i64 number"))?,
        StrOrI64::I64(v) => v,
    })
}

fn str_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        U64(u64),
    }

    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v
            .parse::<u64>()
            .map_err(|_| serde::de::Error::custom("failed to parse u64 number"))?,
        StrOrU64::U64(v) => v,
    })
}

fn bool_or_u64<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrU64 {
        Bool(bool),
        U64(u64),
    }

    Ok(match BoolOrU64::deserialize(deserializer)? {
        BoolOrU64::Bool(v) => v,
        BoolOrU64::U64(v) => v != 0,
    })
}

fn str_or_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrF64<'a> {
        Str(&'a str),
        F64(f64),
    }

    Ok(match StrOrF64::deserialize(deserializer)? {
        StrOrF64::Str(v) => v
            .parse::<f64>()
            .map_err(|_| serde::de::Error::custom("failed to parse f64 number"))?,
        StrOrF64::F64(v) => v,
    })
}

macro_rules! impl_try_from_str {
    ($type:ty) => {
        impl TryFrom<&str> for $type {
            type Error = serde_json::Error;
            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                serde_json::from_str(str)
            }
        }
    };
}
//type ATOMICATTRIBUTE = VariantInt8Int16Int32Int64Uint8Uint16Uint32Uint64Float32Float64StringINT8VECINT16VECINT32VECINT64VECUINT8VECUINT16VECUINT32VECUINT64VECFLOATVECDOUBLEVECSTRINGVEC;
type ATTRIBUTEMAP = Vec<PairStringATOMICATTRIBUTE>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FORMAT {
    pub name: String,
    pub r#type: String,
}
impl_try_from_str!(FORMAT);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Acceptoffer {
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
}
impl_try_from_str!(Acceptoffer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Addcolauth {
    pub collection_name: Name,
    pub account_to_add: Name,
}
impl_try_from_str!(Addcolauth);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Addconftoken {
    pub token_contract: Name,
    pub token_symbol: Symbol,
}
impl_try_from_str!(Addconftoken);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Addnotifyacc {
    pub collection_name: Name,
    pub account_to_add: Name,
}
impl_try_from_str!(Addnotifyacc);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Admincoledit {
    pub collection_format_extension: Vec<FORMAT>,
}
impl_try_from_str!(Admincoledit);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Announcedepo {
    pub owner: Name,
    pub symbol_to_announce: Symbol,
}
impl_try_from_str!(Announcedepo);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssetsS {
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub collection_name: Name,
    pub schema_name: Name,
    pub template_id: Int32,
    pub ram_payer: Name,
    pub backed_tokens: Vec<Asset>,
    pub immutable_serialized_data: Vec<Uint8>,
    pub mutable_serialized_data: Vec<Uint8>,
}
impl_try_from_str!(AssetsS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Backasset {
    pub payer: Name,
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub token_to_back: Asset,
}
impl_try_from_str!(Backasset);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BalancesS {
    pub owner: Name,
    pub quantities: Vec<Asset>,
}
impl_try_from_str!(BalancesS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Burnasset {
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
}
impl_try_from_str!(Burnasset);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Canceloffer {
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
}
impl_try_from_str!(Canceloffer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CollectionsS {
    pub collection_name: Name,
    pub author: Name,
    #[serde(deserialize_with = "bool_or_u64")]
    pub allow_notify: Bool,
    pub authorized_accounts: Vec<Name>,
    pub notify_accounts: Vec<Name>,
    #[serde(deserialize_with = "str_or_f64")]
    pub market_fee: Float64,
    pub serialized_data: Vec<Uint8>,
}
impl_try_from_str!(CollectionsS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConfigS {
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_counter: Uint64,
    pub template_counter: Int32,
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_counter: Uint64,
    pub collection_format: Vec<FORMAT>,
    pub supported_tokens: Vec<ExtendedSymbol>,
}
impl_try_from_str!(ConfigS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Createcol {
    pub author: Name,
    pub collection_name: Name,
    pub allow_notify: Bool,
    pub authorized_accounts: Vec<Name>,
    pub notify_accounts: Vec<Name>,
    pub market_fee: Float64,
    pub data: ATTRIBUTEMAP,
}
impl_try_from_str!(Createcol);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Createoffer {
    pub sender: Name,
    pub recipient: Name,
    pub sender_asset_ids: Vec<Uint64>,
    pub recipient_asset_ids: Vec<Uint64>,
    pub memo: String,
}
impl_try_from_str!(Createoffer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Createschema {
    pub authorized_creator: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub schema_format: Vec<FORMAT>,
}
impl_try_from_str!(Createschema);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Createtempl {
    pub authorized_creator: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub transferable: Bool,
    pub burnable: Bool,
    pub max_supply: Uint32,
    pub immutable_data: ATTRIBUTEMAP,
}
impl_try_from_str!(Createtempl);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Declineoffer {
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
}
impl_try_from_str!(Declineoffer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExtendedSymbol {
    pub sym: Symbol,
    pub contract: Name,
}
impl_try_from_str!(ExtendedSymbol);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Extendschema {
    pub authorized_editor: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub schema_format_extension: Vec<FORMAT>,
}
impl_try_from_str!(Extendschema);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Forbidnotify {
    pub collection_name: Name,
}
impl_try_from_str!(Forbidnotify);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Hydraload {
    pub payload: Vec<HydraloadPayload>,
}
impl_try_from_str!(Hydraload);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HydraloadPayload {
    pub table_name: Name,
    pub scope: Name,
    pub row_data: Bytes,
}
impl_try_from_str!(HydraloadPayload);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Init {
}
impl_try_from_str!(Init);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Locktemplate {
    pub authorized_editor: Name,
    pub collection_name: Name,
    pub template_id: Int32,
}
impl_try_from_str!(Locktemplate);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Logbackasset {
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub backed_token: Asset,
}
impl_try_from_str!(Logbackasset);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Logburnasset {
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub collection_name: Name,
    pub schema_name: Name,
    pub template_id: Int32,
    pub backed_tokens: Vec<Asset>,
    pub old_immutable_data: ATTRIBUTEMAP,
    pub old_mutable_data: ATTRIBUTEMAP,
    pub asset_ram_payer: Name,
}
impl_try_from_str!(Logburnasset);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Logmint {
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub authorized_minter: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub template_id: Int32,
    pub new_asset_owner: Name,
    pub immutable_data: ATTRIBUTEMAP,
    pub mutable_data: ATTRIBUTEMAP,
    pub backed_tokens: Vec<Asset>,
    pub immutable_template_data: ATTRIBUTEMAP,
}
impl_try_from_str!(Logmint);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Lognewoffer {
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
    pub sender: Name,
    pub recipient: Name,
    pub sender_asset_ids: Vec<Uint64>,
    pub recipient_asset_ids: Vec<Uint64>,
    pub memo: String,
}
impl_try_from_str!(Lognewoffer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Lognewtempl {
    pub template_id: Int32,
    pub authorized_creator: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub transferable: Bool,
    pub burnable: Bool,
    pub max_supply: Uint32,
    pub immutable_data: ATTRIBUTEMAP,
}
impl_try_from_str!(Lognewtempl);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Logsetdata {
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub old_data: ATTRIBUTEMAP,
    pub new_data: ATTRIBUTEMAP,
}
impl_try_from_str!(Logsetdata);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Logtransfer {
    pub collection_name: Name,
    pub from: Name,
    pub to: Name,
    pub asset_ids: Vec<String>,
    pub memo: String,
}
impl_try_from_str!(Logtransfer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mintasset {
    pub authorized_minter: Name,
    pub collection_name: Name,
    pub schema_name: Name,
    pub template_id: Int32,
    pub new_asset_owner: Name,
    pub immutable_data: ATTRIBUTEMAP,
    pub mutable_data: ATTRIBUTEMAP,
    pub tokens_to_back: Vec<Asset>,
}
impl_try_from_str!(Mintasset);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OffersS {
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
    pub sender: Name,
    pub recipient: Name,
    pub sender_asset_ids: Vec<Uint64>,
    pub recipient_asset_ids: Vec<Uint64>,
    pub memo: String,
    pub ram_payer: Name,
}
impl_try_from_str!(OffersS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PairStringATOMICATTRIBUTE {
    pub key: String,
    //pub value: ATOMICATTRIBUTE,
}
impl_try_from_str!(PairStringATOMICATTRIBUTE);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Payofferram {
    pub payer: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub offer_id: Uint64,
}
impl_try_from_str!(Payofferram);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Remcolauth {
    pub collection_name: Name,
    pub account_to_remove: Name,
}
impl_try_from_str!(Remcolauth);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Remnotifyacc {
    pub collection_name: Name,
    pub account_to_remove: Name,
}
impl_try_from_str!(Remnotifyacc);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SchemasS {
    pub schema_name: Name,
    pub format: Vec<FORMAT>,
}
impl_try_from_str!(SchemasS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setassetdata {
    pub authorized_editor: Name,
    pub asset_owner: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub asset_id: Uint64,
    pub new_mutable_data: ATTRIBUTEMAP,
}
impl_try_from_str!(Setassetdata);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setcoldata {
    pub collection_name: Name,
    pub data: ATTRIBUTEMAP,
}
impl_try_from_str!(Setcoldata);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setmarketfee {
    pub collection_name: Name,
    pub market_fee: Float64,
}
impl_try_from_str!(Setmarketfee);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setversion {
    pub new_version: String,
}
impl_try_from_str!(Setversion);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TemplatesS {
    pub template_id: Int32,
    pub schema_name: Name,
    pub transferable: Bool,
    pub burnable: Bool,
    pub max_supply: Uint32,
    pub issued_supply: Uint32,
    pub immutable_serialized_data: Vec<Uint8>,
}
impl_try_from_str!(TemplatesS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TokenconfigsS {
    pub standard: Name,
    pub version: String,
}
impl_try_from_str!(TokenconfigsS);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
    pub from: Name,
    pub to: Name,
    pub asset_ids: Vec<String>,
    pub memo: String,
}
impl_try_from_str!(Transfer);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Withdraw {
    pub owner: Name,
    pub token_to_withdraw: Asset,
}
impl_try_from_str!(Withdraw);


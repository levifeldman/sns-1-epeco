use ic_cdk_macros::{query};
use serde_bytes::ByteBuf;
use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize}
    }
};


pub type Key = String;

#[derive(CandidType, Deserialize)]
pub struct StoreArg {
    pub key: Key,
    pub content_type: String,
    pub content_encoding: String,
    pub content: ByteBuf,
    pub sha256: Option<ByteBuf>,
    pub aliased: Option<bool>,
}



#[query]
pub fn validate_store_file_payload(_q: StoreArg) -> Result<String, String> {
    Ok("Payload looks good!".to_string())
}











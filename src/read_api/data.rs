use serde::{Deserialize, Serialize};

use crate::collections::Creator;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorsResponse {
    pub jsonrpc: String,
    pub id: u8,
    pub result: CreatorsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatorsResult {
    pub items: Vec<Asset>,
    pub limit: u64,
    pub page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub authorities: Vec<Authority>,
    pub compression: Compression,
    pub content: Content,
    pub creators: Vec<Creator>,
    pub grouping: Vec<String>,
    pub id: String,
    pub interface: String,
    pub mutable: bool,
    pub ownership: Ownership,
    pub royalty: Royalty,
    pub supply: Supply,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authority {
    pub address: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compression {
    pub asset_hash: String,
    pub compressed: bool,
    pub creator_hash: String,
    pub data_hash: String,
    pub eligible: bool,
    pub leaf_id: u32,
    pub seq: u32,
    pub tree: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub files: Vec<File>,
    pub json_uri: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub mime: Option<String>,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub attributes: Option<Vec<Attribute>>,
    pub description: Option<String>,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ownership {
    pub delegate: Option<String>,
    pub delegated: bool,
    pub frozen: bool,
    pub owner: String,
    pub ownership_model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Royalty {
    pub basis_points: u16,
    pub locked: bool,
    pub percent: f32,
    pub primary_sale_happened: bool,
    pub royalty_model: String,
    pub target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supply {
    pub edition_nonce: u8,
    pub print_current_supply: u32,
    pub print_max_supply: u32,
}

use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetType {
    Observation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub asset_id: String,
    pub asset_type: AssetType,
    pub schema_version: u32,
    pub canonical_payload: Value,
    pub content_hash: String,
    pub canonicalization_policy_version: String,
    pub created_at_ms: i64,
}

impl Asset {
    pub fn new(asset_type: AssetType, schema_version: u32, canonical_payload: Value) -> Self {
        let content_hash = blake3::hash(canonical_payload.to_string().as_bytes()).to_hex().to_string();

        Self {
            asset_id: content_hash.clone(),
            asset_type,
            schema_version,
            canonical_payload,
            content_hash,
            canonicalization_policy_version: "v1".to_owned(),
            created_at_ms: 0,
        }
    }
}

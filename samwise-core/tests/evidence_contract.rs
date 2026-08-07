use samwise_core::evidence::{Asset, AssetType};

#[test]
fn evidence_module_is_public() {
    let _ = Asset::new(AssetType::Observation, 1, serde_json::json!({"x": 1}));
}

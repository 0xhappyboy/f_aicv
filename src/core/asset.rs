//! Asset structures for AI Canvas.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Asset ID to metadata map.
pub type AssetMap = HashMap<String, AssetMetadata>;
/// Category ID to category map.
pub type AssetCategoryMap = HashMap<String, AssetCategory>;
/// Asset metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
impl Default for AssetMetadata {
    fn default() -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id: String::new(),
            name: String::new(),
            image: None,
            category: None,
            category_id: None,
            description: String::new(),
            prompt: String::new(),
            file_path: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
/// Asset category.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCategory {
    pub id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub expanded: bool,
    #[serde(default)]
    pub asset_ids: Vec<String>,
}
fn default_true() -> bool {
    true
}
impl Default for AssetCategory {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            expanded: true,
            asset_ids: Vec::new(),
        }
    }
}

//! Canvas metadata structure.
use super::asset::{AssetCategoryMap, AssetMap};
use super::episode::EpisodeMap;
use serde::{Deserialize, Serialize};
/// Canvas viewport state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewportState {
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub offset_x: f64,
    #[serde(default)]
    pub offset_y: f64,
}
fn default_scale() -> f64 {
    1.0
}
/// Root metadata of an AI Canvas session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CanvasMetadata {
    pub session_id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    /// All episodes of this session.
    #[serde(default)]
    pub episodes: EpisodeMap,
    /// Display order of episode ids.
    #[serde(default)]
    pub episode_stack: Vec<String>,
    /// Asset metadata map.
    #[serde(default)]
    pub assets: AssetMap,
    /// Asset category map.
    #[serde(default)]
    pub asset_categories: AssetCategoryMap,
    /// Display order of asset category ids.
    #[serde(default)]
    pub asset_category_stack: Vec<String>,
    /// Last saved viewport.
    #[serde(default)]
    pub viewport: Option<ViewportState>,
}
impl CanvasMetadata {
    /// Creates a new empty metadata object.
    pub fn new(session_id: &str, title: &str, description: &str) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            session_id: session_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            created_at: now.clone(),
            updated_at: now,
            episodes: std::collections::HashMap::new(),
            episode_stack: Vec::new(),
            assets: std::collections::HashMap::new(),
            asset_categories: std::collections::HashMap::new(),
            asset_category_stack: Vec::new(),
            viewport: None,
        }
    }
    /// Returns the total number of episodes.
    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }
    /// Returns the total number of shots across all episodes.
    pub fn total_shot_count(&self) -> usize {
        self.episodes
            .values()
            .map(|ep| {
                ep.current()
                    .map(|data| {
                        data.scenes
                            .values()
                            .map(|sc| sc.current().map(|d| d.shots.len()).unwrap_or(0))
                            .sum::<usize>()
                    })
                    .unwrap_or(0)
            })
            .sum()
    }
    /// Returns the total number of assets.
    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }
}

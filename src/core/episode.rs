//! Episode structures for AI Canvas.
use super::scene::SceneMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Episode ID to episode map.
pub type EpisodeMap = HashMap<String, Episode>;
/// Version number to episode data map.
pub type EpisodeVersionMap = HashMap<u64, EpisodeVersionData>;
/// Episode data for a specific version.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeVersionData {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub episode_number: u32,
    #[serde(default)]
    pub scenes: SceneMap,
    #[serde(default)]
    pub scene_stack: Vec<String>,
}
/// Episode container with version support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub historical_versions: Vec<u64>,
    #[serde(default)]
    pub current_version: u64,
    #[serde(default)]
    pub version_map: EpisodeVersionMap,
    #[serde(default)]
    pub inherited_from: Vec<String>,
}
impl Default for Episode {
    fn default() -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id: String::new(),
            created_at: now.clone(),
            updated_at: now,
            historical_versions: Vec::new(),
            current_version: 0,
            version_map: HashMap::new(),
            inherited_from: Vec::new(),
        }
    }
}
impl Episode {
    /// Returns the current version data.
    pub fn current(&self) -> Option<&EpisodeVersionData> {
        self.version_map.get(&self.current_version)
    }
    /// Returns a specific version.
    pub fn version(&self, version: u64) -> Option<&EpisodeVersionData> {
        self.version_map.get(&version)
    }
    /// Returns the shot count for the current version.
    pub fn shot_count(&self) -> usize {
        self.current()
            .map(|data| {
                data.scenes
                    .values()
                    .map(|sc| sc.current().map(|d| d.shots.len()).unwrap_or(0))
                    .sum()
            })
            .unwrap_or(0)
    }
}

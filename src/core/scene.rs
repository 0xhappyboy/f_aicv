//! Scene structures for AI Canvas.
use super::shot::ShotMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Scene ID to scene map.
pub type SceneMap = HashMap<String, Scene>;
/// Version number to scene data map.
pub type SceneVersionMap = HashMap<u64, SceneVersionData>;
/// Scene data for a specific version.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneVersionData {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub shots: ShotMap,
}
/// Scene container with version support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub historical_versions: Vec<u64>,
    #[serde(default)]
    pub current_version: u64,
    #[serde(default)]
    pub version_map: SceneVersionMap,
}
impl Default for Scene {
    fn default() -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id: String::new(),
            created_at: now.clone(),
            updated_at: now,
            historical_versions: Vec::new(),
            current_version: 0,
            version_map: HashMap::new(),
        }
    }
}
impl Scene {
    /// Returns the current version data.
    pub fn current(&self) -> Option<&SceneVersionData> {
        self.version_map.get(&self.current_version)
    }
    /// Returns a specific version.
    pub fn version(&self, version: u64) -> Option<&SceneVersionData> {
        self.version_map.get(&version)
    }
}

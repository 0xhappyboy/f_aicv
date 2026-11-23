//! Shot structures for AI Canvas.
use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Shot ID to shot map.
pub type ShotMap = HashMap<String, Shot>;
/// Version number to shot data map.
pub type ShotVersionMap = HashMap<u64, ShotVersionData>;
/// Shot data for a specific version.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShotVersionData {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub scene_number: u32,
    #[serde(default)]
    pub shot_number: u32,
    #[serde(default)]
    pub shot_size: ShotSize,
    #[serde(default)]
    pub camera_move: CameraMove,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera_speed: Option<CameraSpeed>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera_direction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zoom_intensity: Option<f64>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub dialogue: String,
    #[serde(default)]
    pub duration: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageTaskInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoTaskInfo>,
    #[serde(default)]
    pub image_count: u64,
    #[serde(default)]
    pub video_count: u64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub asset_ids: Vec<String>,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub prev_shot_ids: Vec<String>,
    #[serde(default)]
    pub scene_box_x: f64,
    #[serde(default)]
    pub scene_box_y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<ShotAudio>,
}
impl Default for ShotVersionData {
    fn default() -> Self {
        Self {
            title: String::new(),
            scene_number: 1,
            shot_number: 1,
            shot_size: ShotSize::default(),
            camera_move: CameraMove::default(),
            camera_speed: Some(CameraSpeed::default()),
            camera_direction: Some(String::new()),
            zoom_intensity: Some(1.0),
            description: String::new(),
            dialogue: String::new(),
            duration: 4.5,
            image: None,
            video: None,
            image_count: 0,
            video_count: 0,
            x: 0.0,
            y: 0.0,
            width: 320.0,
            height: 280.0,
            asset_ids: Vec::new(),
            prompt: String::new(),
            prev_shot_ids: Vec::new(),
            scene_box_x: 0.0,
            scene_box_y: 0.0,
            audio: None,
        }
    }
}
/// Shot container with version support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shot {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub historical_versions: Vec<u64>,
    #[serde(default)]
    pub current_version: u64,
    #[serde(default)]
    pub version_map: ShotVersionMap,
}
impl Default for Shot {
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
impl Shot {
    /// Returns the current version data.
    pub fn current(&self) -> Option<&ShotVersionData> {
        self.version_map.get(&self.current_version)
    }
    /// Returns a specific version.
    pub fn version(&self, version: u64) -> Option<&ShotVersionData> {
        self.version_map.get(&version)
    }
}

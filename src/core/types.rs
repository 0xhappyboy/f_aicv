//! Shared type definitions for AI Canvas structures.
use serde::{Deserialize, Serialize};
/// Shot size enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShotSize {
    ExtremeLongShot,
    LongShot,
    FullShot,
    #[default]
    MediumShot,
    MediumCloseUp,
    CloseUp,
    ExtremeCloseUp,
    BigCloseUp,
}
/// Camera movement enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CameraMove {
    #[default]
    Static,
    PushIn,
    PullOut,
    Pan,
    Truck,
    Follow,
    Arc,
    Crane,
    PushInArc,
    FollowArc,
    CranePan,
    Handheld,
    DollyZoom,
    OneTake,
}
/// Camera speed enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CameraSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
}
/// Task status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    #[default]
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}
/// Image generation task info.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTaskInfo {
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hippox_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_task_id: Option<String>,
    #[serde(default)]
    pub download_urls: Vec<String>,
    pub prompt: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    #[serde(default)]
    pub file_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
/// Video generation task info.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoTaskInfo {
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hippox_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    pub prompt: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
/// Audio generation task info.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTaskInfo {
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hippox_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    pub prompt: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f32>,
}
/// Dialogue detail for audio generation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogueDetail {
    pub text: String,
    pub speaker: String,
    pub voice: String,
    pub emotion: String,
    pub language: String,
}
/// Background music detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BgmDetail {
    pub prompt: String,
    pub mood: String,
}
/// Sound effect detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfxDetail {
    pub prompt: String,
    pub timing: f64,
}
/// Ambient sound detail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbientDetail {
    pub prompt: String,
    pub volume: f64,
}
/// Audio bundle for a shot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShotAudio {
    pub dialogue: Vec<DialogueDetail>,
    pub bgm: BgmDetail,
    pub sfx: Vec<SfxDetail>,
    pub ambient: AmbientDetail,
}

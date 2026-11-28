//! Unified read-only API for extracted AICV session directories.
use crate::core::metadata::CanvasMetadata;
use crate::io::reader::{extract_metadata, read_archive};
use std::path::{Path, PathBuf};
/// Read-only API for a session directory that has already been extracted from an .aicv archive.
pub struct AicvSessionReader {
    session_dir: PathBuf,
    metadata: CanvasMetadata,
}
impl AicvSessionReader {
    /// Open a session directory and load its metadata.
    pub fn open<P: AsRef<Path>>(session_dir: P) -> Result<Self, String> {
        let session_dir = session_dir.as_ref().to_path_buf();
        let metadata_path = session_dir.join("workspace").join("metadata.json");
        if !metadata_path.exists() {
            return Err(format!(
                "metadata.json not found in {}",
                session_dir.display()
            ));
        }
        let content = std::fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read metadata.json: {}", e))?;
        let metadata: CanvasMetadata = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse metadata.json: {}", e))?;
        Ok(Self {
            session_dir,
            metadata,
        })
    }
    /// Open from an .aicv archive file (metadata only; file resolution will not work).
    pub fn from_archive<P: AsRef<Path>>(archive_path: P) -> Result<Self, String> {
        let package =
            read_archive(archive_path).map_err(|e| format!("Failed to read archive: {}", e))?;
        let metadata =
            extract_metadata(&package).map_err(|e| format!("Failed to extract metadata: {}", e))?;
        Ok(Self {
            session_dir: PathBuf::new(),
            metadata,
        })
    }
    /// Get the full session metadata.
    pub fn metadata(&self) -> &CanvasMetadata {
        &self.metadata
    }
    /// Get the session directory path.
    pub fn session_dir(&self) -> &Path {
        &self.session_dir
    }
    /// Get the ordered list of episode ids.
    pub fn episode_ids(&self) -> Vec<String> {
        self.metadata.episode_stack.clone()
    }
    /// Get all episodes.
    pub fn episodes(&self) -> &crate::core::episode::EpisodeMap {
        &self.metadata.episodes
    }
    /// Get a single episode by id.
    pub fn episode(&self, episode_id: &str) -> Option<&crate::core::episode::Episode> {
        self.metadata.episodes.get(episode_id)
    }
    /// Get the current version data of an episode.
    pub fn episode_current(
        &self,
        episode_id: &str,
    ) -> Option<&crate::core::episode::EpisodeVersionData> {
        self.metadata.episodes.get(episode_id)?.current()
    }
    /// Get a specific version of an episode.
    pub fn episode_version(
        &self,
        episode_id: &str,
        version: u64,
    ) -> Option<&crate::core::episode::EpisodeVersionData> {
        self.metadata.episodes.get(episode_id)?.version(version)
    }
    /// Get all version numbers of an episode.
    pub fn episode_versions(&self, episode_id: &str) -> Vec<u64> {
        self.metadata
            .episodes
            .get(episode_id)
            .map(|ep| ep.historical_versions.clone())
            .unwrap_or_default()
    }
    /// Get all scenes of an episode (current version), ordered by scene_stack.
    pub fn scenes(&self, episode_id: &str) -> Vec<&crate::core::scene::Scene> {
        let mut result = Vec::new();
        if let Some(episode) = self.metadata.episodes.get(episode_id) {
            if let Some(data) = episode.current() {
                for scene_id in &data.scene_stack {
                    if let Some(scene) = data.scenes.get(scene_id) {
                        result.push(scene);
                    }
                }
            }
        }
        result
    }
    /// Get all scenes of a specific episode version, ordered by scene_stack.
    pub fn scenes_at_version(
        &self,
        episode_id: &str,
        episode_version: u64,
    ) -> Vec<&crate::core::scene::Scene> {
        let mut result = Vec::new();
        if let Some(episode) = self.metadata.episodes.get(episode_id) {
            if let Some(data) = episode.version(episode_version) {
                for scene_id in &data.scene_stack {
                    if let Some(scene) = data.scenes.get(scene_id) {
                        result.push(scene);
                    }
                }
            }
        }
        result
    }
    /// Get a single scene by id.
    pub fn scene(&self, episode_id: &str, scene_id: &str) -> Option<&crate::core::scene::Scene> {
        self.metadata
            .episodes
            .get(episode_id)?
            .current()?
            .scenes
            .get(scene_id)
    }
    /// Get the current version data of a scene.
    pub fn scene_current(
        &self,
        episode_id: &str,
        scene_id: &str,
    ) -> Option<&crate::core::scene::SceneVersionData> {
        self.scene(episode_id, scene_id)?.current()
    }
    /// Get a specific version data of a scene.
    pub fn scene_version(
        &self,
        episode_id: &str,
        scene_id: &str,
        version: u64,
    ) -> Option<&crate::core::scene::SceneVersionData> {
        self.scene(episode_id, scene_id)?.version(version)
    }
    /// Get all version numbers of a scene.
    pub fn scene_versions(&self, episode_id: &str, scene_id: &str) -> Vec<u64> {
        self.scene(episode_id, scene_id)
            .map(|s| s.historical_versions.clone())
            .unwrap_or_default()
    }
    /// Get all shots of a scene (current version), ordered by shot_number.
    pub fn scene_shots(&self, episode_id: &str, scene_id: &str) -> Vec<&crate::core::shot::Shot> {
        let mut result = Vec::new();
        if let Some(scene) = self.scene(episode_id, scene_id) {
            if let Some(data) = scene.current() {
                let mut shots: Vec<_> = data.shots.values().collect();
                shots.sort_by_key(|s| s.current().map(|d| d.shot_number).unwrap_or(0));
                result = shots;
            }
        }
        result
    }
    /// Get all shots of a scene at a specific scene version, ordered by shot_number.
    pub fn scene_shots_at_version(
        &self,
        episode_id: &str,
        scene_id: &str,
        scene_version: u64,
    ) -> Vec<&crate::core::shot::Shot> {
        let mut result = Vec::new();
        if let Some(scene) = self.scene(episode_id, scene_id) {
            if let Some(data) = scene.version(scene_version) {
                let mut shots: Vec<_> = data.shots.values().collect();
                shots.sort_by_key(|s| s.current().map(|d| d.shot_number).unwrap_or(0));
                result = shots;
            }
        }
        result
    }
    /// Get a single shot by id (searches all episodes and scenes).
    pub fn shot(&self, shot_id: &str) -> Option<&crate::core::shot::Shot> {
        for episode in self.metadata.episodes.values() {
            if let Some(data) = episode.current() {
                for scene in data.scenes.values() {
                    if let Some(scene_data) = scene.current() {
                        if let Some(shot) = scene_data.shots.get(shot_id) {
                            return Some(shot);
                        }
                    }
                }
            }
        }
        None
    }
    /// Get the current version data of a shot.
    pub fn shot_current(&self, shot_id: &str) -> Option<&crate::core::shot::ShotVersionData> {
        self.shot(shot_id)?.current()
    }
    /// Get a specific version of a shot.
    pub fn shot_version(
        &self,
        shot_id: &str,
        version: u64,
    ) -> Option<&crate::core::shot::ShotVersionData> {
        self.shot(shot_id)?.version(version)
    }
    /// Get all version numbers of a shot.
    pub fn shot_versions(&self, shot_id: &str) -> Vec<u64> {
        self.shot(shot_id)
            .map(|s| s.historical_versions.clone())
            .unwrap_or_default()
    }
    /// Get the shot base directory: <root>/workspace/shots/<shot_id>/
    fn shot_base_dir(&self, shot_id: &str) -> PathBuf {
        self.session_dir
            .join("workspace")
            .join("shots")
            .join(shot_id)
    }
    /// Get the version directory for a shot using the observed layout:
    /// <root>/workspace/shots/<shot_id>/v<shot_version>/
    fn shot_version_dir_simple(&self, shot_id: &str, shot_version: u64) -> PathBuf {
        self.shot_base_dir(shot_id)
            .join(format!("v{}", shot_version))
    }
    /// Get the version directory for a shot using the canonical layout:
    /// <root>/workspace/shots/<shot_id>/e<ep>/s<sc>/v<shot>/
    fn shot_version_dir_full(
        &self,
        shot_id: &str,
        episode_version: u64,
        scene_version: u64,
        shot_version: u64,
    ) -> PathBuf {
        self.shot_base_dir(shot_id)
            .join(format!("e{}", episode_version))
            .join(format!("s{}", scene_version))
            .join(format!("v{}", shot_version))
    }
    /// Resolve the (episode_version, scene_version, shot_version) triple for a shot.
    fn resolve_version_triple(&self, shot_id: &str) -> Option<(u64, u64, u64)> {
        for episode in self.metadata.episodes.values() {
            let episode_version = episode.current_version;
            if let Some(episode_data) = episode.current() {
                for scene in episode_data.scenes.values() {
                    let scene_version = scene.current_version;
                    if let Some(scene_data) = scene.current() {
                        if let Some(shot) = scene_data.shots.get(shot_id) {
                            let shot_version = shot.current_version;
                            return Some((episode_version, scene_version, shot_version));
                        }
                    }
                }
            }
        }
        None
    }
    /// Get the latest generated image file path for a shot (current version).
    pub fn shot_image_path(&self, shot_id: &str) -> Option<String> {
        let shot = self.shot(shot_id)?;
        let shot_version = shot.current_version;
        self.shot_image_path_at_version(shot_id, shot_version)
    }
    /// Get the image file path for a shot at a specific shot version.
    pub fn shot_image_path_at_version(&self, shot_id: &str, shot_version: u64) -> Option<String> {
        let shot_data = self.shot(shot_id)?.version(shot_version)?;
        let _image_task = shot_data.image.as_ref()?;
        // Layout 1 (observed): <root>/workspace/shots/<shot_id>/v<ver>/source.png
        let simple_dir = self.shot_version_dir_simple(shot_id, shot_version);
        let source_png = simple_dir.join("source.png");
        if source_png.exists() {
            return Some(source_png.to_string_lossy().to_string());
        }
        // Layout 2 (canonical): e<ep>/s<sc>/v<shot>/image_<count>.png
        if let Some((ep_ver, sc_ver, _)) = self.resolve_version_triple(shot_id) {
            let full_dir = self.shot_version_dir_full(shot_id, ep_ver, sc_ver, shot_version);
            let count = shot_data.image_count.max(1);
            let path = full_dir.join(format!("image_{}.png", count));
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
            if let Some(found) = self.find_latest_image(&full_dir) {
                return Some(found);
            }
        }
        // Fallback: scan the shot directory recursively for any image file
        self.find_first_image_recursive(&self.shot_base_dir(shot_id))
    }
    /// Get all image file paths for a shot across all its versions.
    pub fn shot_all_image_paths(&self, shot_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(shot) = self.shot(shot_id) {
            for version in &shot.historical_versions {
                if let Some(path) = self.shot_image_path_at_version(shot_id, *version) {
                    result.push(path);
                }
            }
        }
        result
    }
    /// Get the video file path for a shot (current version).
    pub fn shot_video_path(&self, shot_id: &str) -> Option<String> {
        let shot = self.shot(shot_id)?;
        let shot_version = shot.current_version;
        self.shot_video_path_at_version(shot_id, shot_version)
    }
    /// Get the video file path for a shot at a specific shot version.
    pub fn shot_video_path_at_version(&self, shot_id: &str, shot_version: u64) -> Option<String> {
        let shot_data = self.shot(shot_id)?.version(shot_version)?;
        let video_task = shot_data.video.as_ref()?;
        // Layout 1 (observed): <root>/workspace/shots/<shot_id>/v<ver>/tasks/video/<task_id>/source.mp4
        let simple_dir = self.shot_version_dir_simple(shot_id, shot_version);
        let path = simple_dir
            .join("tasks")
            .join("video")
            .join(&video_task.task_id)
            .join("source.mp4");
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
        // Layout 2 (canonical): e<ep>/s<sc>/v<shot>/tasks/video/<task_id>/source.mp4
        if let Some((ep_ver, sc_ver, _)) = self.resolve_version_triple(shot_id) {
            let full_dir = self.shot_version_dir_full(shot_id, ep_ver, sc_ver, shot_version);
            let path = full_dir
                .join("tasks")
                .join("video")
                .join(&video_task.task_id)
                .join("source.mp4");
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
    /// Get the first frame path of a shot video product (current version).
    pub fn shot_video_first_frame_path(&self, shot_id: &str) -> Option<String> {
        let shot = self.shot(shot_id)?;
        let shot_version = shot.current_version;
        self.shot_video_first_frame_path_at_version(shot_id, shot_version)
    }
    /// Get the first frame path of a shot video at a specific shot version.
    pub fn shot_video_first_frame_path_at_version(
        &self,
        shot_id: &str,
        shot_version: u64,
    ) -> Option<String> {
        let shot_data = self.shot(shot_id)?.version(shot_version)?;
        let video_task = shot_data.video.as_ref()?;
        let simple_dir = self.shot_version_dir_simple(shot_id, shot_version);
        let path = simple_dir
            .join("tasks")
            .join("video")
            .join(&video_task.task_id)
            .join("first_frame.png");
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
        if let Some((ep_ver, sc_ver, _)) = self.resolve_version_triple(shot_id) {
            let full_dir = self.shot_version_dir_full(shot_id, ep_ver, sc_ver, shot_version);
            let path = full_dir
                .join("tasks")
                .join("video")
                .join(&video_task.task_id)
                .join("first_frame.png");
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
    /// Get the last frame path of a shot video product (current version).
    pub fn shot_video_last_frame_path(&self, shot_id: &str) -> Option<String> {
        let shot = self.shot(shot_id)?;
        let shot_version = shot.current_version;
        self.shot_video_last_frame_path_at_version(shot_id, shot_version)
    }
    /// Get the last frame path of a shot video at a specific shot version.
    pub fn shot_video_last_frame_path_at_version(
        &self,
        shot_id: &str,
        shot_version: u64,
    ) -> Option<String> {
        let shot_data = self.shot(shot_id)?.version(shot_version)?;
        let video_task = shot_data.video.as_ref()?;
        let simple_dir = self.shot_version_dir_simple(shot_id, shot_version);
        let path = simple_dir
            .join("tasks")
            .join("video")
            .join(&video_task.task_id)
            .join("last_frame.png");
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
        if let Some((ep_ver, sc_ver, _)) = self.resolve_version_triple(shot_id) {
            let full_dir = self.shot_version_dir_full(shot_id, ep_ver, sc_ver, shot_version);
            let path = full_dir
                .join("tasks")
                .join("video")
                .join(&video_task.task_id)
                .join("last_frame.png");
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
    // ==================== ASSETS ====================
    /// Get all assets of the session.
    pub fn assets(&self) -> Vec<&crate::core::asset::AssetMetadata> {
        self.metadata.assets.values().collect()
    }
    /// Get a single asset by id.
    pub fn asset(&self, asset_id: &str) -> Option<&crate::core::asset::AssetMetadata> {
        self.metadata.assets.get(asset_id)
    }
    /// Get all asset categories, ordered by category stack.
    pub fn asset_categories(&self) -> Vec<&crate::core::asset::AssetCategory> {
        let mut result: Vec<_> = self.metadata.asset_categories.values().collect();
        if !self.metadata.asset_category_stack.is_empty() {
            let stack = &self.metadata.asset_category_stack;
            result.sort_by_key(|c| {
                stack
                    .iter()
                    .position(|id| id == &c.id)
                    .unwrap_or(usize::MAX)
            });
        }
        result
    }
    /// Get assets belonging to a category.
    pub fn category_assets(&self, category_id: &str) -> Vec<&crate::core::asset::AssetMetadata> {
        let mut result = Vec::new();
        if let Some(category) = self.metadata.asset_categories.get(category_id) {
            for asset_id in &category.asset_ids {
                if let Some(asset) = self.metadata.assets.get(asset_id) {
                    result.push(asset);
                }
            }
        }
        result
    }
    // ==================== ASSET FILE PATHS ====================
    /// Get the asset base directory: <root>/workspace/assets/<asset_id>/
    fn asset_dir(&self, asset_id: &str) -> PathBuf {
        self.session_dir
            .join("workspace")
            .join("assets")
            .join(asset_id)
    }
    /// Get the image file path for an asset.
    ///
    /// Asset images are stored as <asset_type>.png inside the asset directory.
    /// Falls back to scanning for any image file if the canonical name is not found.
    pub fn asset_image_path(&self, asset_id: &str) -> Option<String> {
        let dir = self.asset_dir(asset_id);
        let asset = self.metadata.assets.get(asset_id);
        let category = asset.and_then(|a| a.category.as_deref()).unwrap_or("");
        // Try canonical names based on asset category
        let candidates: Vec<&str> = match category {
            "character" => vec!["character.png", "image.png"],
            "prop" => vec!["prop.png", "image.png"],
            "environment" => vec!["environment.png", "image.png"],
            "atmosphere" => vec!["atmosphere.png", "image.png"],
            "lighting" => vec!["lighting.png", "image.png"],
            _ => vec!["image.png"],
        };
        for name in candidates {
            let path = dir.join(name);
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        // Fallback: scan directory for any image file
        self.find_first_image(&dir)
    }
    /// Get the video file path for an asset.
    pub fn asset_video_path(&self, asset_id: &str) -> Option<String> {
        let dir = self.asset_dir(asset_id);
        let candidate = dir.join("products.mp4");
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
        // Fallback: scan for any .mp4 file
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "mp4") {
                    return Some(path.to_string_lossy().to_string());
                }
            }
        }
        None
    }
    /// Get the audio file path for an asset.
    ///
    /// Audio assets are stored as <task_id>.mp3 inside the asset directory.
    pub fn asset_audio_path(&self, asset_id: &str) -> Option<String> {
        let dir = self.asset_dir(asset_id);
        // Try products.mp3 first
        let candidate = dir.join("products.mp3");
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
        // Fallback: scan for any .mp3 file
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "mp3") {
                    return Some(path.to_string_lossy().to_string());
                }
            }
        }
        None
    }
    /// Get the waveform image path for an audio asset.
    ///
    /// Audio assets store the waveform as waveform.png inside the asset directory.
    /// Returns None if the asset is not an audio asset or the waveform does not exist.
    pub fn asset_waveform_path(&self, asset_id: &str) -> Option<String> {
        let asset = self.metadata.assets.get(asset_id)?;
        // Only audio assets have waveform
        if asset.category.as_deref() != Some("audio") {
            return None;
        }
        let dir = self.asset_dir(asset_id);
        let candidate = dir.join("waveform.png");
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
        None
    }
    // ==================== STATISTICS ====================
    /// Get the LLM task statistics file path.
    pub fn llm_task_statistics_path(&self) -> PathBuf {
        self.session_dir.join("workspace").join("statistics.json")
    }
    /// Get the image task statistics file path.
    pub fn image_task_statistics_path(&self) -> PathBuf {
        self.session_dir
            .join("workspace")
            .join("image_task_statistics.json")
    }
    /// Get the audio task statistics file path.
    pub fn audio_task_statistics_path(&self) -> PathBuf {
        self.session_dir
            .join("workspace")
            .join("audio_task_statistics.json")
    }
    /// Get the video task statistics file path.
    pub fn video_task_statistics_path(&self) -> PathBuf {
        self.session_dir
            .join("workspace")
            .join("video_task_statistics.json")
    }
    /// Get the LLM task statistics content as a JSON string.
    pub fn llm_task_statistics(&self) -> Option<String> {
        let path = self.llm_task_statistics_path();
        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }
    /// Get the image task statistics content as a JSON string.
    pub fn image_task_statistics(&self) -> Option<String> {
        let path = self.image_task_statistics_path();
        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }
    /// Get the audio task statistics content as a JSON string.
    pub fn audio_task_statistics(&self) -> Option<String> {
        let path = self.audio_task_statistics_path();
        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }
    /// Get the video task statistics content as a JSON string.
    pub fn video_task_statistics(&self) -> Option<String> {
        let path = self.video_task_statistics_path();
        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }
    // ==================== VIEWPORT ====================
    /// Get the saved canvas viewport.
    pub fn viewport(&self) -> crate::core::metadata::ViewportState {
        self.metadata.viewport.unwrap_or_default()
    }
    // ==================== INTERNAL HELPERS ====================
    /// Scan a directory for image_*.png files and return the one with the highest count.
    fn find_latest_image(&self, dir: &Path) -> Option<String> {
        let entries = std::fs::read_dir(dir).ok()?;
        let mut best: Option<(u64, PathBuf)> = None;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Some(rest) = stem.strip_prefix("image_") {
                    if let Ok(count) = rest.parse::<u64>() {
                        let is_png = path.extension().map_or(false, |e| e == "png");
                        if is_png {
                            match &best {
                                Some((best_count, _)) if count <= *best_count => {}
                                _ => best = Some((count, path)),
                            }
                        }
                    }
                }
            }
        }
        best.map(|(_, p)| p.to_string_lossy().to_string())
    }
    /// Find the first image file in a directory (any .png / .jpg / .jpeg / .webp).
    fn find_first_image(&self, dir: &Path) -> Option<String> {
        let entries = std::fs::read_dir(dir).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "webp" {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }
    /// Recursively find the first image file in a directory tree.
    fn find_first_image_recursive(&self, dir: &Path) -> Option<String> {
        if !dir.exists() {
            return None;
        }
        let mut queue = vec![dir.to_path_buf()];
        while let Some(current) = queue.pop() {
            if let Ok(entries) = std::fs::read_dir(&current) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        queue.push(path);
                    } else {
                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "webp" {
                            return Some(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

//! AI Canvas archive engine.
mod api;
mod core;
mod error;
mod io;
mod pkg;
mod xml;
pub use api::AicvSessionReader;
pub use core::asset::{AssetCategory, AssetCategoryMap, AssetMap, AssetMetadata};
pub use core::episode::{Episode, EpisodeMap, EpisodeVersionData, EpisodeVersionMap};
pub use core::metadata::{CanvasMetadata, ViewportState};
pub use core::scene::{Scene, SceneMap, SceneVersionData, SceneVersionMap};
pub use core::shot::{Shot, ShotMap, ShotVersionData, ShotVersionMap};
pub use core::types::*;
pub use error::{FAicvError, FAicvResult};
pub use io::reader::{
    ArchiveSummary, archive_summary, extract_metadata, extract_to_directory, list_entries,
    read_archive, read_archive_from_bytes,
};
pub use io::writer::{
    create_archive, create_archive_from_metadata, create_archive_with_statistics, write_archive,
};
pub use pkg::{
    AICV_FORMAT_VERSION, FAicvEntry, FAicvEntryType, FAicvPackage, MANIFEST_FILE, METADATA_FILE,
};
use std::path::Path;
/// High-level API for AICV archive operations.
pub struct FAicv;
impl FAicv {
    /// Packs a session directory into an .aicv archive file.
    pub fn pack<P: AsRef<Path>>(session_dir: P, output_path: P) -> FAicvResult<()> {
        create_archive(session_dir, output_path)
    }
    /// Packs a session directory into an .aicv archive file with optional statistics.
    pub fn pack_with_statistics<P: AsRef<Path>>(
        session_dir: P,
        output_path: P,
        llm_statistics: Option<&str>,
        image_statistics: Option<&str>,
        audio_statistics: Option<&str>,
        video_statistics: Option<&str>,
    ) -> FAicvResult<()> {
        create_archive_with_statistics(
            session_dir,
            output_path,
            llm_statistics,
            image_statistics,
            audio_statistics,
            video_statistics,
        )
    }
    /// Unpacks an .aicv archive file into a session directory.
    pub fn unpack<P: AsRef<Path>>(archive_path: P, output_dir: P) -> FAicvResult<CanvasMetadata> {
        extract_to_directory(archive_path, output_dir)
    }
    /// Unpacks an .aicv archive file into a session directory with a new session id.
    pub fn unpack_with_new_id<P: AsRef<Path> + Clone>(
        archive_path: P,
        output_dir: P,
        new_id: &str,
    ) -> FAicvResult<CanvasMetadata> {
        let mut metadata = extract_to_directory(archive_path, output_dir.clone())?;
        let old_id = metadata.session_id.clone();
        if old_id == new_id {
            return Ok(metadata);
        }
        metadata.session_id = new_id.to_string();
        let output_path = output_dir.as_ref();
        let metadata_path = output_path.join("workspace").join("metadata.json");
        if metadata_path.exists() {
            let metadata_json = serde_json::to_string_pretty(&metadata)
                .map_err(|e| FAicvError::Xml(format!("failed to serialize metadata: {}", e)))?;
            std::fs::write(&metadata_path, metadata_json)?;
        }
        Self::replace_session_id_in_directory(output_path, &old_id, new_id)?;
        Ok(metadata)
    }
    /// Recursively replaces all occurrences of old session id with new session id in JSON files.
    fn replace_session_id_in_directory(dir: &Path, old_id: &str, new_id: &str) -> FAicvResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                Self::replace_session_id_in_directory(&path, old_id, new_id)?;
            } else if file_type.is_file() {
                if path.extension().map_or(false, |ext| ext == "json") {
                    Self::replace_session_id_in_file(&path, old_id, new_id)?;
                }
            }
        }
        Ok(())
    }
    /// Replaces all occurrences of old session id with new session id in a single JSON file.
    fn replace_session_id_in_file(file_path: &Path, old_id: &str, new_id: &str) -> FAicvResult<()> {
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return Ok(()),
        };
        let mut json_value: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => return Ok(()),
        };
        let mut modified = Self::replace_in_json_value(&mut json_value, old_id, new_id);
        if let serde_json::Value::String(s) = &json_value {
            if s == old_id {
                json_value = serde_json::Value::String(new_id.to_string());
                modified = true;
            }
        }
        if modified {
            let new_content = serde_json::to_string_pretty(&json_value)
                .map_err(|e| FAicvError::Xml(format!("failed to serialize JSON: {}", e)))?;
            std::fs::write(file_path, new_content)?;
        }
        Ok(())
    }
    /// Recursively replaces old session id with new session id in all string values of a JSON value.
    fn replace_in_json_value(value: &mut serde_json::Value, old_id: &str, new_id: &str) -> bool {
        let mut modified = false;
        match value {
            serde_json::Value::Object(map) => {
                for (_, v) in map.iter_mut() {
                    if Self::replace_in_json_value(v, old_id, new_id) {
                        modified = true;
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr.iter_mut() {
                    if Self::replace_in_json_value(item, old_id, new_id) {
                        modified = true;
                    }
                }
            }
            serde_json::Value::String(s) => {
                if s == old_id {
                    *s = new_id.to_string();
                    modified = true;
                }
            }
            _ => {}
        }
        modified
    }
    /// Reads metadata from an .aicv archive without extracting.
    pub fn read_metadata<P: AsRef<Path>>(archive_path: P) -> FAicvResult<CanvasMetadata> {
        let package = read_archive(archive_path)?;
        extract_metadata(&package)
    }
    /// Returns a summary of the archive contents.
    pub fn summary<P: AsRef<Path>>(archive_path: P) -> FAicvResult<ArchiveSummary> {
        let package = read_archive(archive_path)?;
        Ok(archive_summary(&package))
    }
    /// Validates an .aicv archive file.
    pub fn validate<P: AsRef<Path>>(archive_path: P) -> FAicvResult<bool> {
        let data = std::fs::read(archive_path)?;
        let package = read_archive_from_bytes(&data)?;
        if package.get_entry(METADATA_FILE).is_none() {
            return Ok(false);
        }
        Ok(true)
    }
}

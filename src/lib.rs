//! AI Canvas archive engine.
mod core;
mod error;
mod io;
mod pkg;
mod xml;
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
pub use io::writer::{create_archive, create_archive_from_metadata, write_archive};
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
    /// Unpacks an .aicv archive file into a session directory.
    /// Returns the canvas metadata.
    pub fn unpack<P: AsRef<Path>>(archive_path: P, output_dir: P) -> FAicvResult<CanvasMetadata> {
        extract_to_directory(archive_path, output_dir)
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
        // Check required entries.
        if package.get_entry(METADATA_FILE).is_none() {
            return Ok(false);
        }
        Ok(true)
    }
}

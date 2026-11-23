//! AICV archive reading logic.
use crate::core::metadata::CanvasMetadata;
use crate::pkg::{FAicvEntry, FAicvEntryType, FAicvPackage, MANIFEST_FILE, METADATA_FILE};
use crate::xml::extract_attr;
use crate::{FAicvError, FAicvResult};
use std::io::{Cursor, Read};
use std::path::Path;
use zip::read::ZipArchive;
/// Reads an AICV archive from a file path.
pub fn read_archive<P: AsRef<Path>>(path: P) -> FAicvResult<FAicvPackage> {
    let data = std::fs::read(path)?;
    read_archive_from_bytes(&data)
}
/// Reads an AICV archive from raw bytes.
pub fn read_archive_from_bytes(data: &[u8]) -> FAicvResult<FAicvPackage> {
    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor)?;
    let mut package = FAicvPackage::new();
    let mut manifest_xml: Option<String> = None;
    // First pass: read manifest and collect all entries.
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().unwrap_or_default().to_string();
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;
        if name == MANIFEST_FILE {
            manifest_xml = Some(String::from_utf8_lossy(&buf).to_string());
        } else {
            package.entries.push(FAicvEntry {
                path: name.clone(),
                data: buf,
                entry_type: classify_entry_type(&name),
            });
        }
    }
    // Parse manifest for metadata.
    if let Some(xml) = manifest_xml {
        package.session_id = extract_attr(&xml, "aicv", "sessionId").unwrap_or_default();
        package.version = extract_attr(&xml, "aicv", "version").unwrap_or_default();
        package.created_at = extract_attr(&xml, "aicv", "createdAt").unwrap_or_default();
    }
    Ok(package)
}
/// Extracts canvas metadata from an AICV archive.
pub fn extract_metadata(package: &FAicvPackage) -> FAicvResult<CanvasMetadata> {
    // Try workspace/metadata.json first (actual session structure)
    let metadata_bytes = package
        .get_entry("workspace/metadata.json")
        .or_else(|| package.get_entry(METADATA_FILE))
        .ok_or_else(|| FAicvError::Invalid(format!("metadata.json not found in archive")))?;
    let metadata: CanvasMetadata = serde_json::from_slice(metadata_bytes)
        .map_err(|e| FAicvError::Xml(format!("failed to parse metadata: {}", e)))?;
    Ok(metadata)
}
/// Extracts an AICV archive to a session directory.
pub fn extract_to_directory<P: AsRef<Path>>(
    archive_path: P,
    output_dir: P,
) -> FAicvResult<CanvasMetadata> {
    let package = read_archive(archive_path)?;
    let output_path = output_dir.as_ref();
    // Create output directory if it doesn't exist.
    std::fs::create_dir_all(output_path)?;
    // Extract all entries.
    for entry in &package.entries {
        let entry_path = output_path.join(&entry.path);
        // Ensure parent directory exists.
        if let Some(parent) = entry_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&entry_path, &entry.data)?;
    }
    // Extract and return metadata.
    extract_metadata(&package)
}
/// Lists all entries in an AICV archive by type.
pub fn list_entries(package: &FAicvPackage, entry_type: FAicvEntryType) -> Vec<String> {
    package
        .entries_by_type(entry_type)
        .iter()
        .map(|e| e.path.clone())
        .collect()
}
/// Returns a summary of the archive contents.
pub fn archive_summary(package: &FAicvPackage) -> ArchiveSummary {
    let mut summary = ArchiveSummary {
        session_id: package.session_id.clone(),
        version: package.version.clone(),
        total_entries: package.entries.len(),
        image_count: 0,
        video_count: 0,
        audio_count: 0,
        metadata_count: 0,
        total_size_bytes: 0,
    };
    for entry in &package.entries {
        summary.total_size_bytes += entry.data.len();
        match entry.entry_type {
            FAicvEntryType::Image => summary.image_count += 1,
            FAicvEntryType::Video => summary.video_count += 1,
            FAicvEntryType::Audio => summary.audio_count += 1,
            FAicvEntryType::Metadata => summary.metadata_count += 1,
            _ => {}
        }
    }
    summary
}
/// Summary of archive contents.
#[derive(Debug, Clone, Default)]
pub struct ArchiveSummary {
    pub session_id: String,
    pub version: String,
    pub total_entries: usize,
    pub image_count: usize,
    pub video_count: usize,
    pub audio_count: usize,
    pub metadata_count: usize,
    pub total_size_bytes: usize,
}
/// Classifies an entry based on its file extension.
fn classify_entry_type(path: &str) -> FAicvEntryType {
    let lower = path.to_lowercase();
    if lower.ends_with(".json") {
        FAicvEntryType::Metadata
    } else if lower.ends_with(".xml") {
        FAicvEntryType::Manifest
    } else if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".bmp")
        || lower.ends_with(".webp")
    {
        FAicvEntryType::Image
    } else if lower.ends_with(".mp4") || lower.ends_with(".mov") || lower.ends_with(".webm") {
        FAicvEntryType::Video
    } else if lower.ends_with(".mp3") || lower.ends_with(".wav") || lower.ends_with(".flac") {
        FAicvEntryType::Audio
    } else {
        FAicvEntryType::Binary
    }
}

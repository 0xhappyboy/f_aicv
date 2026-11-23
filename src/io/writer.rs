//! AICV archive writing logic.
use crate::core::metadata::CanvasMetadata;
use crate::pkg::{AICV_FORMAT_VERSION, FAicvPackage, MANIFEST_FILE, METADATA_FILE};
use crate::xml::escape_xml;
use crate::{FAicvError, FAicvResult};
use std::io::Write;
use std::path::Path;
use zip::ZipWriter;
use zip::write::FileOptions;
/// Creates an AICV archive from a session directory.
pub fn create_archive<P: AsRef<Path>>(session_dir: P, output_path: P) -> FAicvResult<()> {
    let session_path = session_dir.as_ref();
    if !session_path.exists() {
        return Err(FAicvError::Invalid(format!(
            "session directory not found: {}",
            session_path.display()
        )));
    }
    // Collect all files from session directory.
    let mut package = FAicvPackage::new();
    package.session_id = session_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();
    package.created_at = chrono::Local::now().to_rfc3339();
    collect_files(session_path, session_path, &mut package)?;
    // Write archive.
    write_archive(&package, output_path)
}
/// Creates an AICV archive from a package structure.
pub fn write_archive<P: AsRef<Path>>(package: &FAicvPackage, output_path: P) -> FAicvResult<()> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options: FileOptions<'_, '_, ()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    // Write manifest.
    let manifest = serialize_manifest(package);
    writer.start_file(MANIFEST_FILE, options)?;
    writer.write_all(manifest.as_bytes())?;
    // Write all entries.
    for entry in &package.entries {
        writer.start_file(entry.path.as_str(), options)?;
        writer.write_all(&entry.data)?;
    }
    let cursor = writer.finish()?;
    std::fs::write(output_path, cursor.get_ref())?;
    Ok(())
}
/// Creates an AICV archive from metadata and media files.
pub fn create_archive_from_metadata<P: AsRef<Path>>(
    metadata: &CanvasMetadata,
    media_files: &[(String, Vec<u8>)],
    output_path: P,
) -> FAicvResult<()> {
    let mut package = FAicvPackage::new();
    package.session_id = metadata.session_id.clone();
    package.created_at = chrono::Local::now().to_rfc3339();
    let metadata_json = serde_json::to_string_pretty(metadata)
        .map_err(|e| FAicvError::Xml(format!("failed to serialize metadata: {}", e)))?;
    package.set_entry("workspace/metadata.json", metadata_json.into_bytes());
    for (path, data) in media_files {
        package.set_entry(path, data.clone());
    }
    write_archive(&package, output_path)
}
/// Serializes the manifest XML.
fn serialize_manifest(package: &FAicvPackage) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    out.push_str(&format!(
        "<aicv version=\"{}\" sessionId=\"{}\" createdAt=\"{}\">\n",
        escape_xml(&package.version),
        escape_xml(&package.session_id),
        escape_xml(&package.created_at)
    ));
    out.push_str(&format!(
        "  <entries count=\"{}\">\n",
        package.entries.len()
    ));
    for entry in &package.entries {
        out.push_str(&format!(
            "    <entry path=\"{}\" type=\"{:?}\" size=\"{}\"/>\n",
            escape_xml(&entry.path),
            entry.entry_type,
            entry.data.len()
        ));
    }
    out.push_str("  </entries>\n");
    out.push_str("</aicv>\n");
    out
}
/// Recursively collects files from a directory into the package.
fn collect_files(root: &Path, current: &Path, package: &mut FAicvPackage) -> FAicvResult<()> {
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(root, &path, package)?;
        } else if file_type.is_file() {
            let relative_path = path
                .strip_prefix(root)
                .map_err(|e| FAicvError::Invalid(format!("failed to get relative path: {}", e)))?;
            let archive_path = relative_path.to_string_lossy().replace('\\', "/");
            let data = std::fs::read(&path)?;
            package.set_entry(&archive_path, data);
        }
    }
    Ok(())
}

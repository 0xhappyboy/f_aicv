//! Package-level structures for the AICV archive format.
pub const AICV_FORMAT_VERSION: &str = "1.0";
/// Archive manifest file name.
pub const MANIFEST_FILE: &str = "manifest.xml";
/// Archive metadata file name.
pub const METADATA_FILE: &str = "metadata.json";
/// A single file entry in the archive.
#[derive(Debug, Clone, Default)]
pub struct FAicvEntry {
    /// Archive-relative path.
    pub path: String,
    /// Raw bytes of the entry.
    pub data: Vec<u8>,
    /// Entry type hint.
    pub entry_type: FAicvEntryType,
}
/// Type classification for archive entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FAicvEntryType {
    /// Unknown entry type.
    #[default]
    Unknown,
    /// JSON metadata file.
    Metadata,
    /// XML manifest.
    Manifest,
    /// Image file (png, jpg, etc.).
    Image,
    /// Video file (mp4, etc.).
    Video,
    /// Audio file (mp3, etc.).
    Audio,
    /// Text file.
    Text,
    /// Binary data file.
    Binary,
}
/// The root structure representing an AICV archive.
#[derive(Debug, Clone, Default)]
pub struct FAicvPackage {
    /// Format version string.
    pub version: String,
    /// Session identifier.
    pub session_id: String,
    /// Archive creation timestamp.
    pub created_at: String,
    /// All entries in the archive.
    pub entries: Vec<FAicvEntry>,
}
impl FAicvPackage {
    /// Creates a new empty package.
    pub fn new() -> Self {
        Self {
            version: AICV_FORMAT_VERSION.to_string(),
            session_id: String::new(),
            created_at: String::new(),
            entries: Vec::new(),
        }
    }
    /// Returns the bytes of an entry by archive path, if present.
    pub fn get_entry(&self, path: &str) -> Option<&[u8]> {
        self.entries
            .iter()
            .find(|e| e.path == path)
            .map(|e| e.data.as_slice())
    }
    /// Inserts or replaces an entry.
    pub fn set_entry(&mut self, path: &str, data: Vec<u8>) {
        let entry_type = classify_entry(path);
        if let Some(existing) = self.entries.iter_mut().find(|e| e.path == path) {
            existing.data = data;
            existing.entry_type = entry_type;
        } else {
            self.entries.push(FAicvEntry {
                path: path.to_string(),
                data,
                entry_type,
            });
        }
    }
    /// Removes an entry by archive path.
    pub fn remove_entry(&mut self, path: &str) {
        self.entries.retain(|e| e.path != path);
    }
    /// Returns all entries of a specific type.
    pub fn entries_by_type(&self, entry_type: FAicvEntryType) -> Vec<&FAicvEntry> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }
}
/// Classifies an entry based on its file extension.
fn classify_entry(path: &str) -> FAicvEntryType {
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
        || lower.ends_with(".svg")
    {
        FAicvEntryType::Image
    } else if lower.ends_with(".mp4")
        || lower.ends_with(".mov")
        || lower.ends_with(".webm")
        || lower.ends_with(".avi")
    {
        FAicvEntryType::Video
    } else if lower.ends_with(".mp3")
        || lower.ends_with(".wav")
        || lower.ends_with(".flac")
        || lower.ends_with(".ogg")
    {
        FAicvEntryType::Audio
    } else if lower.ends_with(".txt") || lower.ends_with(".md") || lower.ends_with(".csv") {
        FAicvEntryType::Text
    } else {
        FAicvEntryType::Binary
    }
}

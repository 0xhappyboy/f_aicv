//! Error types for the AI Canvas archive engine.
#[derive(Debug)]
pub enum FAicvError {
    /// I/O failure when reading or writing files.
    Io(std::io::Error),
    /// The archive could not be opened or a required entry is missing.
    Zip(zip::result::ZipError),
    /// The XML content is malformed or does not match the expected schema.
    Xml(String),
    /// The requested operation is not valid for the current archive state.
    Invalid(String),
    /// The archive format version is not supported.
    UnsupportedVersion(String),
}
impl std::fmt::Display for FAicvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FAicvError::Io(e) => write!(f, "io error: {}", e),
            FAicvError::Zip(e) => write!(f, "zip error: {}", e),
            FAicvError::Xml(e) => write!(f, "xml error: {}", e),
            FAicvError::Invalid(e) => write!(f, "invalid operation: {}", e),
            FAicvError::UnsupportedVersion(e) => write!(f, "unsupported version: {}", e),
        }
    }
}
impl std::error::Error for FAicvError {}
impl From<std::io::Error> for FAicvError {
    fn from(e: std::io::Error) -> Self {
        FAicvError::Io(e)
    }
}
impl From<zip::result::ZipError> for FAicvError {
    fn from(e: zip::result::ZipError) -> Self {
        FAicvError::Zip(e)
    }
}
pub type FAicvResult<T> = Result<T, FAicvError>;

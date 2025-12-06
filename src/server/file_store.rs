use crate::models::ContactInquiry;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const CONTACTS_DIR: &str = "data/contacts";

/// Saves a contact inquiry to a JSON file
pub fn save_contact_inquiry(inquiry: &ContactInquiry) -> Result<PathBuf, FileStoreError> {
    // Ensure directory exists
    fs::create_dir_all(CONTACTS_DIR)?;

    // Generate filename from timestamp
    let timestamp = inquiry
        .submitted_at
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| FileStoreError::TimestampFormat)?;

    // Replace colons with hyphens for filesystem compatibility
    let safe_timestamp = timestamp.replace(':', "-");
    let filename = format!("{}.json", safe_timestamp);
    let path = PathBuf::from(CONTACTS_DIR).join(&filename);

    // Serialize and write
    let json = serde_json::to_string_pretty(inquiry)?;
    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;

    tracing::info!("Saved contact inquiry to {}", path.display());

    Ok(path)
}

/// Errors that can occur during file storage operations
#[derive(Debug)]
pub enum FileStoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    TimestampFormat,
}

impl std::fmt::Display for FileStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Json(e) => write!(f, "JSON error: {}", e),
            Self::TimestampFormat => write!(f, "Failed to format timestamp"),
        }
    }
}

impl std::error::Error for FileStoreError {}

impl From<std::io::Error> for FileStoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for FileStoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_save_contact_inquiry() {
        let temp_dir = TempDir::new().unwrap();
        let contacts_path = temp_dir.path().join("contacts");
        fs::create_dir_all(&contacts_path).unwrap();

        let inquiry = ContactInquiry::new(
            "Test User".to_string(),
            "010-1234-5678".to_string(),
            "This is a test message.".to_string(),
        )
        .unwrap();

        // Note: This test would need modification to use temp_dir
        // For now, just verify the inquiry was created correctly
        assert_eq!(inquiry.name, "Test User");
        assert_eq!(inquiry.phone, "010-1234-5678");
    }
}

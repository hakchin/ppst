use crate::models::ContactInquiry;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

const CONTACTS_DIR: &str = "data/contacts";

/// Saves a contact inquiry to a JSON file in the default directory
pub async fn save_contact_inquiry(inquiry: &ContactInquiry) -> Result<PathBuf, FileStoreError> {
    save_contact_inquiry_to(inquiry, Path::new(CONTACTS_DIR)).await
}

/// Internal implementation that saves to a specific directory (for testing)
pub async fn save_contact_inquiry_to(
    inquiry: &ContactInquiry,
    base_dir: &Path,
) -> Result<PathBuf, FileStoreError> {
    // Ensure directory exists
    fs::create_dir_all(base_dir).await?;

    // Generate filename from timestamp
    let timestamp = inquiry
        .submitted_at
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| FileStoreError::TimestampFormat)?;

    // Replace colons with hyphens for filesystem compatibility
    let safe_timestamp = timestamp.replace(':', "-");
    let filename = format!("{}.json", safe_timestamp);
    let path = base_dir.join(&filename);

    // Serialize and write
    let json = serde_json::to_string_pretty(inquiry)?;
    let mut file = fs::File::create(&path).await?;
    file.write_all(json.as_bytes()).await?;

    tracing::info!("Saved contact inquiry to {}", path.display());

    Ok(path)
}

/// Errors that can occur during file storage operations
#[derive(Debug, Error)]
pub enum FileStoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Failed to format timestamp")]
    TimestampFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_inquiry_creation() {
        let inquiry = ContactInquiry::new(
            "Test User".to_string(),
            "010-1234-5678".to_string(),
            "This is a test message.".to_string(),
        );

        assert!(inquiry.is_ok());
        let inquiry = inquiry.unwrap();
        assert_eq!(inquiry.name, "Test User");
        // Phone number is stored with digits only (hyphens removed)
        assert_eq!(inquiry.phone, "01012345678");
    }

    #[tokio::test]
    async fn test_save_contact_inquiry() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path();

        let inquiry = ContactInquiry::new(
            "Async Tester".to_string(),
            "010-9876-5432".to_string(),
            "Testing async save".to_string(),
        )
        .unwrap();

        let result = save_contact_inquiry_to(&inquiry, base_path).await;
        assert!(result.is_ok());

        let file_path = result.unwrap();
        assert!(file_path.exists());
        assert!(file_path.starts_with(base_path));

        // Verify content
        let content = tokio::fs::read_to_string(file_path).await.unwrap();
        let saved_inquiry: ContactInquiry = serde_json::from_str(&content).unwrap();
        assert_eq!(saved_inquiry.name, "Async Tester");
    }
}

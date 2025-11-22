use crate::error::Result;
use crate::models::contact::ContactInquiry;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Save contact inquiry to JSON file in data/contacts/ directory
/// Filename format: YYYY-MM-DDTHH-MM-SS-mmmZ.json
pub fn save_contact_inquiry(inquiry: &ContactInquiry) -> Result<()> {
    // Ensure data/contacts directory exists
    let contacts_dir = Path::new("data/contacts");
    if !contacts_dir.exists() {
        fs::create_dir_all(contacts_dir)?;
    }

    // Create filename from inquiry ID (which is already a timestamp)
    let filename = format!("{}.json", inquiry.id);
    let filepath = contacts_dir.join(filename);

    // Serialize to JSON - errors automatically converted via ?
    let json = serde_json::to_string_pretty(inquiry)?;

    // Write to file
    let mut file = fs::File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_save_contact_inquiry_creates_file() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Change working directory temporarily for this test
        let original_dir = env::current_dir().expect("Failed to get current dir");
        env::set_current_dir(temp_dir.path()).expect("Failed to change dir");

        // Create test inquiry
        let inquiry = ContactInquiry {
            id: "2025-01-22T12-00-00-000Z".to_string(),
            submitted_at: Utc::now(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            subject: Some("Test Subject".to_string()),
            message: "This is a test message for integration testing.".to_string(),
            user_agent: Some("TestAgent/1.0".to_string()),
            ip_address: Some("127.0.0.1".to_string()),
        };

        // Save the inquiry
        let result = save_contact_inquiry(&inquiry);
        assert!(result.is_ok(), "Failed to save inquiry: {:?}", result.err());

        // Verify the file was created
        let expected_file = Path::new("data/contacts")
            .join(format!("{}.json", inquiry.id));
        assert!(
            expected_file.exists(),
            "File was not created at {:?}",
            expected_file
        );

        // Verify the file contents
        let contents = fs::read_to_string(&expected_file)
            .expect("Failed to read saved file");
        let saved_inquiry: ContactInquiry = serde_json::from_str(&contents)
            .expect("Failed to parse saved JSON");

        assert_eq!(saved_inquiry.id, inquiry.id);
        assert_eq!(saved_inquiry.name, inquiry.name);
        assert_eq!(saved_inquiry.email, inquiry.email);
        assert_eq!(saved_inquiry.message, inquiry.message);

        // Restore original directory
        env::set_current_dir(original_dir).expect("Failed to restore dir");
    }

    #[test]
    fn test_save_contact_inquiry_creates_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let original_dir = env::current_dir().expect("Failed to get current dir");
        env::set_current_dir(temp_dir.path()).expect("Failed to change dir");

        // Verify directory doesn't exist initially
        assert!(!Path::new("data/contacts").exists());

        let inquiry = ContactInquiry {
            id: "2025-01-22T12-30-00-000Z".to_string(),
            submitted_at: Utc::now(),
            name: "Another User".to_string(),
            email: "another@example.com".to_string(),
            subject: None,
            message: "Testing directory creation.".to_string(),
            user_agent: None,
            ip_address: None,
        };

        // Save should create the directory
        save_contact_inquiry(&inquiry).expect("Failed to save");

        // Verify directory was created
        assert!(Path::new("data/contacts").exists());
        assert!(Path::new("data/contacts").is_dir());

        env::set_current_dir(original_dir).expect("Failed to restore dir");
    }
}

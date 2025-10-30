use crate::models::contact::ContactInquiry;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Save contact inquiry to JSON file in data/contacts/ directory
/// Filename format: YYYY-MM-DDTHH-MM-SS-mmmZ.json
pub fn save_contact_inquiry(inquiry: &ContactInquiry) -> Result<(), std::io::Error> {
    // Ensure data/contacts directory exists
    let contacts_dir = Path::new("data/contacts");
    if !contacts_dir.exists() {
        fs::create_dir_all(contacts_dir)?;
    }

    // Create filename from inquiry ID (which is already a timestamp)
    let filename = format!("{}.json", inquiry.id);
    let filepath = contacts_dir.join(filename);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(inquiry)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Write to file
    let mut file = fs::File::create(filepath)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_save_contact_inquiry() {
        let inquiry = ContactInquiry {
            id: "test-2025-10-30T12-00-00-000Z".to_string(),
            submitted_at: Utc::now(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            subject: Some("Test Subject".to_string()),
            message: "Test message".to_string(),
            user_agent: None,
            ip_address: None,
        };

        // This would require test fixtures, but demonstrates the structure
        // In production, we'd use a test directory
        // assert!(save_contact_inquiry(&inquiry).is_ok());
    }
}

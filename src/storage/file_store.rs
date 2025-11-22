use crate::error::Result;
use crate::models::contact::ContactInquiry;
use serde_json::Value;
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

fn normalize_submitted_at(value: &mut Value) {
    if let Some(obj) = value.as_object_mut() {
        if let Some(ts) = obj.get_mut("submitted_at") {
            if let Some(s) = ts.as_str() {
                if s.starts_with("+00") {
                    let normalized = s.trim_start_matches("+00").to_string();
                    *ts = Value::String(normalized);
                }
            }
        }
    }
}

pub fn read_contacts_as_values() -> Result<Vec<Value>> {
    let contacts_dir = Path::new("data/contacts");
    if !contacts_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<_> = fs::read_dir(contacts_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Some(name) = e.file_name().to_str() {
                name.ends_with(".json")
            } else {
                false
            }
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let mut values = Vec::new();
    for entry in entries {
        let s = fs::read_to_string(entry.path())?;
        let mut v: Value = serde_json::from_str(&s)?;
        normalize_submitted_at(&mut v);
        values.push(v);
    }
    Ok(values)
}

pub fn export_contacts_ndjson() -> Result<String> {
    let values = read_contacts_as_values()?;
    let mut out = String::new();
    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&serde_json::to_string(v)?);
    }
    Ok(out)
}

pub fn export_contacts_json_array_pretty() -> Result<String> {
    let values = read_contacts_as_values()?;
    let s = serde_json::to_string_pretty(&values)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;
    use time::OffsetDateTime;

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
            submitted_at: OffsetDateTime::now_utc(),
            name: "Test User".to_string(),
            email: Some("test@example.com".to_string()),
            phone: None,
            subject: Some("Test Subject".to_string()),
            message: "This is a test message for integration testing.".to_string(),
            user_agent: Some("TestAgent/1.0".to_string()),
            ip_address: Some("127.0.0.1".to_string()),
        };

        // Save the inquiry
        let result = save_contact_inquiry(&inquiry);
        assert!(result.is_ok(), "Failed to save inquiry: {:?}", result.err());

        // Verify the file was created
        let expected_file = Path::new("data/contacts").join(format!("{}.json", inquiry.id));
        assert!(
            expected_file.exists(),
            "File was not created at {:?}",
            expected_file
        );

        // Verify the file contents
        let contents = fs::read_to_string(&expected_file).expect("Failed to read saved file");
        let saved_inquiry: ContactInquiry =
            serde_json::from_str(&contents).expect("Failed to parse saved JSON");

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
            submitted_at: OffsetDateTime::now_utc(),
            name: "Another User".to_string(),
            email: None,
            phone: None,
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

#[cfg(test)]
mod export_tests {
    use super::*;

    #[test]
    fn normalize_function_handles_expanded_year_prefix() {
        let mut v: Value = serde_json::json!({
            "id":"2025-11-22T15-50-22-729Z",
            "submitted_at":"+002025-11-22T15:50:22.729296000Z",
            "name":"Unit",
            "email":null,
            "phone":null,
            "subject":null,
            "message":"M",
            "user_agent":null,
            "ip_address":null
        });
        normalize_submitted_at(&mut v);
        let s = serde_json::to_string(&v).unwrap();
        assert!(s.contains("\"submitted_at\":\"2025-11-22T15:50:22.729296000Z\""));
    }
}

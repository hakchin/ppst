use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Represents a contact form submission from a website visitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInquiry {
    /// Unique identifier (ISO 8601 timestamp)
    pub id: String,
    /// Submission timestamp
    #[serde(with = "time::serde::iso8601")]
    pub submitted_at: OffsetDateTime,
    /// Visitor's full name (required)
    pub name: String,
    /// Visitor's email address (required, validated)
    pub email: String,
    /// Optional subject line
    pub subject: Option<String>,
    /// Message content (required)
    pub message: String,
    /// User agent string (for analytics/debugging)
    pub user_agent: Option<String>,
    /// IP address (optional, for rate limiting)
    pub ip_address: Option<String>,
}

/// Represents raw form data from HTTP POST
#[derive(Debug, Deserialize)]
pub struct ContactFormInput {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

/// Validation error with list of error messages
#[derive(Debug)]
pub struct ValidationError {
    pub errors: Vec<String>,
}

impl ContactFormInput {
    /// Validate form input
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Name validation
        if self.name.trim().is_empty() {
            errors.push("Name is required".to_string());
        } else if self.name.len() > 100 {
            errors.push("Name must be less than 100 characters".to_string());
        }

        // Email validation
        if self.email.trim().is_empty() {
            errors.push("Email is required".to_string());
        } else if !is_valid_email(&self.email) {
            errors.push("Email format is invalid".to_string());
        } else if self.email.len() > 255 {
            errors.push("Email must be less than 255 characters".to_string());
        }

        // Subject validation (optional)
        if let Some(subject) = &self.subject
            && subject.len() > 200
        {
            errors.push("Subject must be less than 200 characters".to_string());
        }

        // Message validation
        if self.message.trim().is_empty() {
            errors.push("Message is required".to_string());
        } else if self.message.len() > 5000 {
            errors.push("Message must be less than 5000 characters".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError { errors })
        }
    }

    /// Convert form input to ContactInquiry with server-generated fields
    pub fn into_inquiry(
        self,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<ContactInquiry, ValidationError> {
        // Validate fields
        self.validate()?;

        // Generate timestamp and ID
        let now = OffsetDateTime::now_utc();
        let id = format_timestamp_id(&now);

        Ok(ContactInquiry {
            id,
            submitted_at: now,
            name: self.name.trim().to_string(),
            email: self.email.trim().to_lowercase(),
            subject: self.subject.filter(|s| !s.trim().is_empty()),
            message: self.message.trim().to_string(),
            user_agent,
            ip_address,
        })
    }
}

/// Basic email validation (simplified RFC 5322)
fn is_valid_email(email: &str) -> bool {
    use regex::Regex;
    use std::sync::LazyLock;

    static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("Invalid email regex pattern")
    });

    EMAIL_REGEX.is_match(email)
}

/// Generate a filesystem-safe timestamp ID
fn format_timestamp_id(dt: &OffsetDateTime) -> String {
    use time::macros::format_description;
    let format =
        format_description!("[year]-[month]-[day]T[hour]-[minute]-[second]-[subsecond digits:3]Z");
    dt.format(&format).unwrap_or_else(|_| "unknown".to_string())
}

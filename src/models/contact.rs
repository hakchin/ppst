use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Contact inquiry submitted through the contact form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInquiry {
    pub name: String,
    pub phone: String,
    pub message: String,
    #[serde(with = "time::serde::rfc3339")]
    pub submitted_at: OffsetDateTime,
}

impl ContactInquiry {
    /// Creates a new contact inquiry with the current timestamp
    pub fn new(name: String, phone: String, message: String) -> Result<Self, ValidationError> {
        // Validate name
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(ValidationError::EmptyField("name"));
        }
        if name.len() > 100 {
            return Err(ValidationError::TooLong("name", 100));
        }

        // Validate phone and extract digits only
        let phone = phone.trim().to_string();
        if phone.is_empty() {
            return Err(ValidationError::EmptyField("phone"));
        }
        if phone.len() > 20 {
            return Err(ValidationError::TooLong("phone", 20));
        }
        // Remove hyphens and keep only digits for storage
        let phone: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

        // Validate message
        let message = message.trim().to_string();
        if message.is_empty() {
            return Err(ValidationError::EmptyField("message"));
        }
        if message.len() > 5000 {
            return Err(ValidationError::TooLong("message", 5000));
        }

        Ok(Self {
            name,
            phone,
            message,
            submitted_at: OffsetDateTime::now_utc(),
        })
    }
}

/// Validation errors for contact inquiry
#[derive(Debug, Clone)]
pub enum ValidationError {
    EmptyField(&'static str),
    TooLong(&'static str, usize),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => {
                let field_ko = match *field {
                    "name" => "이름",
                    "phone" => "휴대폰 번호",
                    "message" => "내용",
                    _ => field,
                };
                write!(f, "{}을(를) 입력해주세요.", field_ko)
            }
            Self::TooLong(field, max) => {
                let field_ko = match *field {
                    "name" => "이름",
                    "phone" => "휴대폰 번호",
                    "message" => "내용",
                    _ => field,
                };
                write!(f, "{}은(는) {}자 이하여야 합니다.", field_ko, max)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_contact_inquiry() {
        let inquiry = ContactInquiry::new(
            "John Doe".to_string(),
            "010-1234-5678".to_string(),
            "Hello, I have a question.".to_string(),
        );
        assert!(inquiry.is_ok());
    }

    #[test]
    fn test_empty_name_rejected() {
        let inquiry = ContactInquiry::new(
            "".to_string(),
            "010-1234-5678".to_string(),
            "Hello".to_string(),
        );
        assert!(matches!(inquiry, Err(ValidationError::EmptyField("name"))));
    }

    #[test]
    fn test_empty_phone_rejected() {
        let inquiry = ContactInquiry::new(
            "John".to_string(),
            "".to_string(),
            "Hello".to_string(),
        );
        assert!(matches!(inquiry, Err(ValidationError::EmptyField("phone"))));
    }
}

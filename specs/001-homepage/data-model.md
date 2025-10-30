# Data Model: Academy Homepage

**Feature**: 001-homepage
**Date**: 2025-10-30
**Source**: Derived from [spec.md](spec.md) Key Entities section

## Overview

This document defines the data structures for the Academy Homepage feature. Since the project follows the "Static Content Architecture" principle (no databases), all data is either:
1. **Static content**: Hard-coded in templates or configuration files
2. **User-generated content**: Stored as files (contact form submissions)

## Entity Definitions

### 1. Contact Inquiry

**Purpose**: Represents a contact form submission from a website visitor

**Storage**: File-based (JSON files in `data/contacts/` directory)

**Rust Structure**:
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInquiry {
    /// Unique identifier (ISO 8601 timestamp)
    pub id: String,

    /// Submission timestamp
    pub submitted_at: DateTime<Utc>,

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
```

**Validation Rules**:
- `name`: Required, non-empty after trimming, max 100 characters
- `email`: Required, valid email format (RFC 5322 subset), max 255 characters
- `subject`: Optional, max 200 characters if provided
- `message`: Required, non-empty after trimming, max 5000 characters
- `submitted_at`: Auto-generated server-side
- `id`: Auto-generated from timestamp (e.g., "2025-10-30T14-23-45-123Z")

**File Storage Format**:
```json
{
  "id": "2025-10-30T14-23-45-123Z",
  "submitted_at": "2025-10-30T14:23:45.123Z",
  "name": "John Doe",
  "email": "john@example.com",
  "subject": "Question about courses",
  "message": "I would like to know more about your programming courses...",
  "user_agent": "Mozilla/5.0...",
  "ip_address": "192.168.1.1"
}
```

**Filename Convention**: `{timestamp}.json` (e.g., `2025-10-30T14-23-45-123Z.json`)

**State Transitions**: None (immutable once created)

---

### 2. Academy Information

**Purpose**: Represents static content about the academy displayed on the homepage

**Storage**: Template-based (hard-coded in Askama templates or config file)

**Rust Structure**:
```rust
#[derive(Debug, Clone)]
pub struct AcademyInfo {
    /// Academy name
    pub name: String,

    /// Tagline/slogan
    pub tagline: String,

    /// Mission statement (1-3 paragraphs)
    pub mission: String,

    /// Logo path (relative to static/ directory)
    pub logo_path: String,

    /// List of programs offered
    pub programs: Vec<Program>,

    /// List of instructors
    pub instructors: Vec<Instructor>,
}

#[derive(Debug, Clone)]
pub struct Program {
    /// Program name
    pub name: String,

    /// Short description
    pub description: String,

    /// Icon or image path (optional)
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Instructor {
    /// Instructor full name
    pub name: String,

    /// Credentials (e.g., "Ph.D. in Computer Science")
    pub credentials: String,

    /// Profile photo path
    pub photo_path: String,

    /// Short bio (1-2 sentences)
    pub bio: String,
}
```

**Storage Options**:

**Option A: Hard-coded in template context** (simplest for initial implementation)
```rust
impl AcademyInfo {
    pub fn default() -> Self {
        Self {
            name: "PPST Academy".to_string(),
            tagline: "Excellence in Programming Education".to_string(),
            mission: "Our mission is...".to_string(),
            logo_path: "/static/images/logo.png".to_string(),
            programs: vec![
                Program {
                    name: "Web Development".to_string(),
                    description: "Learn modern web technologies...".to_string(),
                    icon_path: Some("/static/images/web-icon.svg".to_string()),
                },
                // More programs...
            ],
            instructors: vec![
                Instructor {
                    name: "Jane Smith".to_string(),
                    credentials: "Ph.D. in Computer Science".to_string(),
                    photo_path: "/static/images/instructors/jane.jpg".to_string(),
                    bio: "20 years of experience in...".to_string(),
                },
                // More instructors...
            ],
        }
    }
}
```

**Option B: TOML configuration file** (better for future editing)
```toml
# config/academy.toml
[academy]
name = "PPST Academy"
tagline = "Excellence in Programming Education"
mission = """
Our mission is to provide world-class programming education...
"""
logo_path = "/static/images/logo.png"

[[programs]]
name = "Web Development"
description = "Learn modern web technologies..."
icon_path = "/static/images/web-icon.svg"

[[programs]]
name = "Data Science"
description = "Master data analysis and machine learning..."
icon_path = "/static/images/data-icon.svg"

[[instructors]]
name = "Jane Smith"
credentials = "Ph.D. in Computer Science"
photo_path = "/static/images/instructors/jane.jpg"
bio = "20 years of experience in software engineering..."
```

**Recommendation**: Start with Option A (hard-coded), migrate to Option B (TOML) when content management becomes a priority.

**Validation**: None required (static content controlled by developers)

---

## Form Input Model

### Contact Form Input

**Purpose**: Represents raw form data from HTTP POST

**Rust Structure**:
```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContactFormInput {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

impl ContactFormInput {
    /// Convert form input to ContactInquiry with server-generated fields
    pub fn into_inquiry(
        self,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<ContactInquiry, ValidationError> {
        // Validate fields
        self.validate()?;

        // Generate timestamp and ID
        let now = Utc::now();
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

    /// Validate form input
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Name validation
        if self.name.trim().is_empty() {
            errors.push("Name is required");
        } else if self.name.len() > 100 {
            errors.push("Name must be less than 100 characters");
        }

        // Email validation
        if self.email.trim().is_empty() {
            errors.push("Email is required");
        } else if !is_valid_email(&self.email) {
            errors.push("Email format is invalid");
        } else if self.email.len() > 255 {
            errors.push("Email must be less than 255 characters");
        }

        // Subject validation (optional)
        if let Some(subject) = &self.subject {
            if subject.len() > 200 {
                errors.push("Subject must be less than 200 characters");
            }
        }

        // Message validation
        if self.message.trim().is_empty() {
            errors.push("Message is required");
        } else if self.message.len() > 5000 {
            errors.push("Message must be less than 5000 characters");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::new(errors))
        }
    }
}

#[derive(Debug)]
pub struct ValidationError {
    pub errors: Vec<String>,
}

impl ValidationError {
    fn new(errors: Vec<&str>) -> Self {
        Self {
            errors: errors.into_iter().map(String::from).collect(),
        }
    }
}
```

---

## Helper Functions

### Email Validation

```rust
/// Basic email validation (simplified RFC 5322)
fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    re.is_match(email)
}
```

### Timestamp ID Generation

```rust
/// Generate a filesystem-safe timestamp ID
fn format_timestamp_id(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H-%M-%S-%3fZ").to_string()
}
```

---

## Entity Relationships

```
┌─────────────────────┐
│   AcademyInfo       │ (Static, in-memory)
├─────────────────────┤
│ - name              │
│ - tagline           │
│ - mission           │
│ - logo_path         │
│ - programs: Vec     │────┐
│ - instructors: Vec  │────┤
└─────────────────────┘    │
                            │
         ┌──────────────────┴──────────────────┐
         │                                     │
         ▼                                     ▼
┌─────────────────┐               ┌─────────────────┐
│    Program      │               │   Instructor    │
├─────────────────┤               ├─────────────────┤
│ - name          │               │ - name          │
│ - description   │               │ - credentials   │
│ - icon_path     │               │ - photo_path    │
└─────────────────┘               │ - bio           │
                                  └─────────────────┘

┌─────────────────────────┐
│   ContactInquiry        │ (Persisted as JSON files)
├─────────────────────────┤
│ - id (timestamp)        │
│ - submitted_at          │
│ - name                  │
│ - email                 │
│ - subject (optional)    │
│ - message               │
│ - user_agent            │
│ - ip_address            │
└─────────────────────────┘

         ▲
         │ created from
         │
┌─────────────────────────┐
│  ContactFormInput       │ (Request data)
├─────────────────────────┤
│ - name                  │
│ - email                 │
│ - subject (optional)    │
│ - message               │
└─────────────────────────┘
```

---

## File Storage Structure

```
data/
└── contacts/
    ├── 2025-10-30T14-23-45-123Z.json
    ├── 2025-10-30T15-10-22-456Z.json
    ├── 2025-10-30T16-05-33-789Z.json
    └── .gitkeep
```

**Notes**:
- Each contact submission is a separate JSON file
- Filenames use timestamp for chronological ordering
- `.gitkeep` ensures directory exists in version control
- Directory should have appropriate permissions (writable by web server)
- Consider adding `.gitignore` for `*.json` to avoid committing user data

---

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
regex = "1.10"
```

---

## Summary

The data model is intentionally simple, adhering to the "Static Content Architecture" principle:

- **Academy Information**: Static, managed through code/config
- **Contact Inquiries**: File-based persistence, one JSON file per submission
- **No databases**: All storage is file-based or in-memory
- **Type safety**: Rust's type system ensures correctness at compile time
- **Validation**: Server-side validation before persistence

This model supports all functional requirements (FR-001 through FR-015) while maintaining constitutional compliance.

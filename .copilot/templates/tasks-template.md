# Task Breakdown Template

**Tasks ID**: tasks-{number}-{short-name}  
**Plan ID**: plan-{number}-{short-name}  
**Spec ID**: spec-{number}-{short-name}  
**Created**: {date}  
**Status**: Not Started | In Progress | Completed

## Summary

### Overview
Brief description of the task breakdown and overall goal.

### Progress Tracking

| Category | Completed | Total | Progress |
|----------|-----------|-------|----------|
| Setup | 0 | X | ░░░░░░░░░░ 0% |
| Backend | 0 | X | ░░░░░░░░░░ 0% |
| Frontend | 0 | X | ░░░░░░░░░░ 0% |
| Testing | 0 | X | ░░░░░░░░░░ 0% |
| Documentation | 0 | X | ░░░░░░░░░░ 0% |
| Deployment | 0 | X | ░░░░░░░░░░ 0% |
| **TOTAL** | **0** | **X** | ░░░░░░░░░░ **0%** |

### Effort Estimate
- **Minimum**: X hours
- **Expected**: Y hours
- **Maximum**: Z hours

### Critical Path
Tasks that must be completed in sequence (blocking tasks):
- TASK-001 → TASK-002 → TASK-005 → TASK-010 → TASK-015

## Tasks

### Setup Tasks

#### TASK-001: Initialize Project Dependencies

- **Category**: Setup
- **Priority**: P0 (Critical)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: None
- **Assignee**: -

**Description**:
Add required dependencies to `Cargo.toml` and verify they compile correctly.

**Acceptance Criteria**:
- [ ] Dependencies added to `Cargo.toml`
- [ ] `cargo build` completes successfully
- [ ] No conflicting versions

**Implementation Notes**:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
askama = "0.12"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
```

---

#### TASK-002: Create Directory Structure

- **Category**: Setup
- **Priority**: P0 (Critical)
- **Estimate**: 0.5 hours
- **Status**: Not Started
- **Dependencies**: TASK-001
- **Assignee**: -

**Description**:
Create necessary directories for handlers, models, templates, and data storage.

**Acceptance Criteria**:
- [ ] `src/handlers/` directory created
- [ ] `src/models/` directory created
- [ ] `templates/partials/` directory created
- [ ] `data/{entity}/` directory created
- [ ] `.gitkeep` files added to empty directories

**Implementation Notes**:
```bash
mkdir -p src/handlers src/models templates/partials data/{entity}
touch data/{entity}/.gitkeep
```

---

### Backend Tasks

#### TASK-003: Define Data Model

- **Category**: Backend
- **Priority**: P1 (High)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-001
- **Assignee**: -

**Description**:
Create the data model struct with Serde serialization in `src/models/{entity}.rs`.

**Acceptance Criteria**:
- [ ] Struct defined with all required fields
- [ ] Derive macros added (Debug, Serialize, Deserialize)
- [ ] Validation methods implemented
- [ ] Unit tests for validation written

**Implementation Notes**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub field1: String,
    pub field2: String,
    pub timestamp: String,
}

impl Entity {
    pub fn validate(&self) -> Result<(), String> {
        // Validation logic
    }
}
```

---

#### TASK-004: Implement File Storage Utility

- **Category**: Backend
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-003
- **Assignee**: -

**Description**:
Create utility functions for saving JSON files with proper timestamps.

**Acceptance Criteria**:
- [ ] Function to generate ISO 8601 timestamps
- [ ] Function to save struct to JSON file
- [ ] Error handling for file operations
- [ ] Directory creation if not exists
- [ ] Unit tests for storage operations

**Implementation Notes**:
```rust
use chrono::Utc;
use std::fs;
use std::path::Path;

pub fn save_entity(data: &Entity) -> Result<String, std::io::Error> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%S-%3fZ");
    let dir = "data/entity";
    fs::create_dir_all(dir)?;
    let filename = format!("{}/{}.json", dir, timestamp);
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&filename, json)?;
    Ok(filename)
}
```

---

#### TASK-005: Create Axum Handler

- **Category**: Backend
- **Priority**: P1 (High)
- **Estimate**: 3 hours
- **Status**: Not Started
- **Dependencies**: TASK-003, TASK-004
- **Assignee**: -

**Description**:
Implement Axum handlers for GET and POST requests in `src/handlers/{feature}.rs`.

**Acceptance Criteria**:
- [ ] GET handler renders page template
- [ ] POST handler receives form data
- [ ] POST handler validates input
- [ ] POST handler saves to file
- [ ] POST handler renders success/error template
- [ ] Proper error handling throughout

**Implementation Notes**:
```rust
use axum::{
    extract::{State, Form},
    response::Html,
};
use askama::Template;

pub async fn handle_get() -> Result<Html<String>, AppError> {
    // Implementation
}

pub async fn handle_post(
    Form(input): Form<InputStruct>
) -> Result<Html<String>, AppError> {
    // Implementation
}
```

---

#### TASK-006: Register Routes

- **Category**: Backend
- **Priority**: P1 (High)
- **Estimate**: 0.5 hours
- **Status**: Not Started
- **Dependencies**: TASK-005
- **Assignee**: -

**Description**:
Add routes to `src/routes.rs` and verify they're accessible.

**Acceptance Criteria**:
- [ ] Routes added to router
- [ ] GET route accessible
- [ ] POST route accessible
- [ ] Routes tested with curl/browser

**Implementation Notes**:
```rust
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/path", get(handlers::feature::handle_get))
        .route("/path", post(handlers::feature::handle_post))
}
```

---

### Frontend Tasks

#### TASK-007: Create Base Template

- **Category**: Frontend
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-002
- **Assignee**: -

**Description**:
Create or update the base template with proper HTML structure and Tailwind CSS.

**Acceptance Criteria**:
- [ ] Semantic HTML5 structure
- [ ] Proper meta tags for responsiveness
- [ ] Tailwind CSS linked
- [ ] Accessibility attributes (lang, aria)
- [ ] Block definitions for content extension

**Implementation Notes**:
```html
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}{% endblock %}</title>
    <link href="/css/output.css" rel="stylesheet">
</head>
<body class="bg-gray-50">
    {% block content %}{% endblock %}
</body>
</html>
```

---

#### TASK-008: Create Main Page Template

- **Category**: Frontend
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-007
- **Assignee**: -

**Description**:
Create the main page template that extends the base template.

**Acceptance Criteria**:
- [ ] Extends base template
- [ ] Semantic HTML structure
- [ ] Responsive layout with Tailwind CSS
- [ ] Includes form partial
- [ ] Accessible heading hierarchy

**Implementation Notes**:
```html
{% extends "base.html" %}

{% block title %}Page Title{% endblock %}

{% block content %}
<main class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold mb-6">Heading</h1>
  {% include "partials/form.html" %}
</main>
{% endblock %}
```

---

#### TASK-009: Create Form Partial

- **Category**: Frontend
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-007
- **Assignee**: -

**Description**:
Create reusable form component in `templates/partials/form.html`.

**Acceptance Criteria**:
- [ ] Native HTML form (works without JavaScript)
- [ ] Proper labels and input associations
- [ ] Required field validation
- [ ] Accessible error messages
- [ ] Tailwind CSS styling
- [ ] Mobile responsive

**Implementation Notes**:
```html
<form method="post" action="/endpoint" class="max-w-md mx-auto">
  <div class="mb-4">
    <label for="field" class="block mb-2">Field Label</label>
    <input type="text" id="field" name="field" required
           class="w-full px-3 py-2 border rounded">
  </div>
  <button type="submit" class="bg-blue-600 text-white px-4 py-2 rounded">
    Submit
  </button>
</form>
```

---

#### TASK-010: Add Responsive Styling

- **Category**: Frontend
- **Priority**: P2 (Medium)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-008, TASK-009
- **Assignee**: -

**Description**:
Ensure all components are responsive across breakpoints.

**Acceptance Criteria**:
- [ ] Mobile layout (< 768px) tested
- [ ] Tablet layout (768px-1023px) tested
- [ ] Desktop layout (≥ 1024px) tested
- [ ] Touch targets adequate on mobile (≥ 44px)
- [ ] Text readable on all screen sizes

**Implementation Notes**:
Use Tailwind responsive prefixes: `sm:`, `md:`, `lg:`, `xl:`

---

#### TASK-011: Add HTMX Enhancement (Optional)

- **Category**: Frontend
- **Priority**: P3 (Low)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-009
- **Assignee**: -

**Description**:
If justified in spec, add HTMX attributes for progressive enhancement.

**Acceptance Criteria**:
- [ ] HTMX library included
- [ ] Form submits via HTMX
- [ ] Target element updates without page reload
- [ ] Still works without JavaScript (fallback)
- [ ] Loading states indicated

**Implementation Notes**:
```html
<form hx-post="/endpoint" hx-target="#result" hx-swap="outerHTML">
  <!-- form fields -->
</form>
```

---

### Testing Tasks

#### TASK-012: Write Unit Tests

- **Category**: Testing
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-003, TASK-004
- **Assignee**: -

**Description**:
Write unit tests for data model validation and storage utilities.

**Acceptance Criteria**:
- [ ] Validation tests cover all rules
- [ ] Storage tests verify file creation
- [ ] Edge cases tested
- [ ] All tests passing

**Implementation Notes**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_valid_input() {
        // Test
    }

    #[test]
    fn test_validation_invalid_input() {
        // Test
    }
}
```

---

#### TASK-013: Write Integration Tests

- **Category**: Testing
- **Priority**: P1 (High)
- **Estimate**: 3 hours
- **Status**: Not Started
- **Dependencies**: TASK-005, TASK-006
- **Assignee**: -

**Description**:
Write integration tests for handler request/response cycles.

**Acceptance Criteria**:
- [ ] GET request test passing
- [ ] POST request with valid data test passing
- [ ] POST request with invalid data test passing
- [ ] File creation verified in tests

**Implementation Notes**:
```rust
#[tokio::test]
async fn test_handler_get() {
    // Test
}

#[tokio::test]
async fn test_handler_post_valid() {
    // Test
}
```

---

#### TASK-014: Manual Testing Checklist

- **Category**: Testing
- **Priority**: P1 (High)
- **Estimate**: 2 hours
- **Status**: Not Started
- **Dependencies**: TASK-006, TASK-010
- **Assignee**: -

**Description**:
Complete manual testing checklist covering all functionality.

**Acceptance Criteria**:
- [ ] Works with JavaScript disabled
- [ ] Keyboard navigation functional
- [ ] Screen reader tested (VoiceOver/NVDA)
- [ ] Mobile browser tested
- [ ] Cross-browser tested (Chrome, Firefox, Safari)
- [ ] Form submission successful
- [ ] Validation errors display correctly
- [ ] File saved in correct location

**Implementation Notes**:
Test in actual browsers, not just development tools.

---

#### TASK-015: Accessibility Audit

- **Category**: Testing
- **Priority**: P2 (Medium)
- **Estimate**: 1.5 hours
- **Status**: Not Started
- **Dependencies**: TASK-010
- **Assignee**: -

**Description**:
Run accessibility tools and verify WCAG 2.1 Level AA compliance.

**Acceptance Criteria**:
- [ ] Lighthouse accessibility score ≥ 90
- [ ] axe DevTools reports no violations
- [ ] Color contrast ratios meet WCAG AA
- [ ] Focus indicators visible
- [ ] ARIA labels correct

**Implementation Notes**:
Use browser DevTools, Lighthouse, and axe DevTools extensions.

---

### Documentation Tasks

#### TASK-016: Add Code Comments

- **Category**: Documentation
- **Priority**: P2 (Medium)
- **Estimate**: 1.5 hours
- **Status**: Not Started
- **Dependencies**: TASK-005, TASK-004
- **Assignee**: -

**Description**:
Add clear, educational comments to code explaining why, not what.

**Acceptance Criteria**:
- [ ] Public functions documented
- [ ] Complex logic explained
- [ ] Educational comments added
- [ ] No redundant comments

**Implementation Notes**:
```rust
/// Validates contact form input.
///
/// We check email format and message length to ensure
/// quality submissions and prevent spam.
pub fn validate(input: &Input) -> Result<(), ValidationError> {
    // Implementation
}
```

---

#### TASK-017: Update README

- **Category**: Documentation
- **Priority**: P2 (Medium)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-006
- **Assignee**: -

**Description**:
Update README.md with new feature information and usage.

**Acceptance Criteria**:
- [ ] Feature description added
- [ ] Usage instructions clear
- [ ] File structure documented
- [ ] Examples provided

---

#### TASK-018: Create Quickstart Guide

- **Category**: Documentation
- **Priority**: P3 (Low)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-017
- **Assignee**: -

**Description**:
Create quickstart guide in `specs/{number}-{name}/quickstart.md`.

**Acceptance Criteria**:
- [ ] Setup steps listed
- [ ] Common tasks documented
- [ ] Troubleshooting section included

---

### Deployment Tasks

#### TASK-019: Build for Production

- **Category**: Deployment
- **Priority**: P0 (Critical)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-015
- **Assignee**: -

**Description**:
Build application for production deployment.

**Acceptance Criteria**:
- [ ] CSS compiled with Tailwind
- [ ] Rust application builds in release mode
- [ ] No warnings in build output
- [ ] Binary tested and working

**Implementation Notes**:
```bash
# Build CSS
tailwindcss -i static/css/input.css -o static/css/output.css --minify

# Build Rust
cargo build --release

# Test
./target/release/ppst
```

---

#### TASK-020: Deploy Application

- **Category**: Deployment
- **Priority**: P0 (Critical)
- **Estimate**: 1 hour
- **Status**: Not Started
- **Dependencies**: TASK-019
- **Assignee**: -

**Description**:
Deploy application to production environment.

**Acceptance Criteria**:
- [ ] Application running in production
- [ ] Data directory writable
- [ ] Static files served correctly
- [ ] Forms working in production

---

## Timeline

| Task ID | Title | Priority | Estimate | Status | Assignee |
|---------|-------|----------|----------|--------|----------|
| TASK-001 | Initialize Project Dependencies | P0 | 1h | Not Started | - |
| TASK-002 | Create Directory Structure | P0 | 0.5h | Not Started | - |
| TASK-003 | Define Data Model | P1 | 1h | Not Started | - |
| TASK-004 | Implement File Storage Utility | P1 | 2h | Not Started | - |
| TASK-005 | Create Axum Handler | P1 | 3h | Not Started | - |
| TASK-006 | Register Routes | P1 | 0.5h | Not Started | - |
| TASK-007 | Create Base Template | P1 | 2h | Not Started | - |
| TASK-008 | Create Main Page Template | P1 | 2h | Not Started | - |
| TASK-009 | Create Form Partial | P1 | 2h | Not Started | - |
| TASK-010 | Add Responsive Styling | P2 | 2h | Not Started | - |
| TASK-011 | Add HTMX Enhancement | P3 | 1h | Not Started | - |
| TASK-012 | Write Unit Tests | P1 | 2h | Not Started | - |
| TASK-013 | Write Integration Tests | P1 | 3h | Not Started | - |
| TASK-014 | Manual Testing Checklist | P1 | 2h | Not Started | - |
| TASK-015 | Accessibility Audit | P2 | 1.5h | Not Started | - |
| TASK-016 | Add Code Comments | P2 | 1.5h | Not Started | - |
| TASK-017 | Update README | P2 | 1h | Not Started | - |
| TASK-018 | Create Quickstart Guide | P3 | 1h | Not Started | - |
| TASK-019 | Build for Production | P0 | 1h | Not Started | - |
| TASK-020 | Deploy Application | P0 | 1h | Not Started | - |

## Notes

- Follow constitution principles strictly
- Each task completion should update status
- Use checkboxes in acceptance criteria to track progress
- Update progress tracking table as tasks complete
- Communicate blockers immediately

---

**Reminder**: This is a living document. Update task statuses and notes as you progress.

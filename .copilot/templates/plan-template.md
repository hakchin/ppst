# Implementation Plan Template

**Plan ID**: plan-{number}-{short-name}  
**Spec ID**: spec-{number}-{short-name}  
**Created**: {date}  
**Status**: Draft | Approved | In Progress | Completed

## Executive Summary

### Goal
Brief description of what this implementation accomplishes.

### Scope
- **In Scope**: What will be implemented
- **Out of Scope**: What won't be implemented
- **Deferred**: What might come later

### Phases Overview
1. Phase 1: {Name} - {Duration estimate}
2. Phase 2: {Name} - {Duration estimate}
3. Phase 3: {Name} - {Duration estimate}
4. Phase 4: {Name} - {Duration estimate}
5. Phase 5: {Name} - {Duration estimate}

**Total Estimated Effort**: {X-Y hours}

## Architecture

### Backend Components

**Handler: `src/handlers/{feature}.rs`**
```rust
// Handler signature example
pub async fn handle_{action}(
    State(state): State<AppState>,
    Form(input): Form<{InputStruct}>
) -> Result<Html<String>, AppError> {
    // Implementation
}
```

**Model: `src/models/{entity}.rs`**
```rust
// Data structure example
#[derive(Debug, Serialize, Deserialize)]
pub struct {Entity} {
    pub field1: String,
    pub field2: String,
    pub timestamp: String,
}
```

**Routes: `src/routes.rs`**
```rust
// Route registration
Router::new()
    .route("/{path}", get(handlers::{feature}::handle_get))
    .route("/{path}", post(handlers::{feature}::handle_post))
```

### Frontend Components

**Main Template: `templates/{page}.html`**
```html
{% extends "base.html" %}

{% block title %}{Page Title}{% endblock %}

{% block content %}
  <!-- Page content -->
{% endblock %}
```

**Partial: `templates/partials/{component}.html`**
```html
<!-- Reusable component -->
<div class="component">
  <!-- Component markup -->
</div>
```

### Data Flow

```text
1. User submits form
   ↓
2. Browser POST to /endpoint
   ↓
3. Axum handler receives Form<InputStruct>
   ↓
4. Handler validates input
   ↓
5. Handler writes JSON to data/{entity}/
   ↓
6. Handler renders success template
   ↓
7. Browser receives complete HTML
```

## Implementation Phases

### Phase 1: Project Setup

**Duration**: {X hours}

**Objectives**:
- Set up project structure
- Add dependencies
- Create base files

**Tasks**:
1. Add dependencies to `Cargo.toml`
2. Create directory structure
3. Set up data storage directories
4. Create base templates

**Deliverables**:
- [ ] Dependencies installed
- [ ] Directory structure created
- [ ] Base templates ready

**Constitution Gate Check**:
- ✅ Server-Side First: N/A (setup phase)
- ✅ Static Content: Data directories created
- ✅ HTML/CSS First: Base templates HTML-only
- ✅ Educational Quality: Clear structure
- ✅ Progressive Enhancement: N/A (setup phase)

### Phase 2: Backend Development

**Duration**: {X hours}

**Objectives**:
- Implement Axum handlers
- Create data models
- Add validation logic
- Implement file storage

**Tasks**:
1. Create `src/models/{entity}.rs`
2. Implement validation functions
3. Create `src/handlers/{feature}.rs`
4. Implement file write operations
5. Register routes in `src/routes.rs`
6. Add error handling

**Deliverables**:
- [ ] Data model defined
- [ ] Handler implemented
- [ ] Validation working
- [ ] File storage working
- [ ] Routes registered
- [ ] Errors handled gracefully

**Constitution Gate Check**:
- ✅ Server-Side First: All logic server-side
- ✅ Static Content: File-based storage only
- ✅ HTML/CSS First: No client-side requirements
- ✅ Educational Quality: Clear, documented code
- ✅ Progressive Enhancement: Server handles all logic

### Phase 3: Frontend Development

**Duration**: {X hours}

**Objectives**:
- Create Askama templates
- Implement HTML structure
- Add Tailwind CSS styling
- Add HTMX enhancements (if needed)
- Ensure responsive design

**Tasks**:
1. Create main page template
2. Create form partial
3. Add Tailwind CSS classes
4. Implement responsive breakpoints
5. Add HTMX attributes (if needed)
6. Test keyboard navigation
7. Verify accessibility

**Deliverables**:
- [ ] Templates created
- [ ] Styling complete
- [ ] Responsive design working
- [ ] HTMX enhancements added (if applicable)
- [ ] Keyboard navigation verified
- [ ] Accessibility checked

**Constitution Gate Check**:
- ✅ Server-Side First: Templates render server-side
- ✅ Static Content: No dynamic data sources
- ✅ HTML/CSS First: Works without JavaScript
- ✅ Educational Quality: Clean, semantic HTML
- ✅ Progressive Enhancement: Base works, HTMX enhances

### Phase 4: Testing & Validation

**Duration**: {X hours}

**Objectives**:
- Write unit tests
- Write integration tests
- Manual testing
- Accessibility testing

**Tasks**:
1. Write validation unit tests
2. Write handler integration tests
3. Test with JavaScript disabled
4. Test keyboard navigation
5. Test screen reader compatibility
6. Test mobile responsiveness
7. Cross-browser testing

**Deliverables**:
- [ ] Unit tests passing
- [ ] Integration tests passing
- [ ] Manual testing complete
- [ ] Accessibility verified
- [ ] Cross-browser compatibility confirmed

**Constitution Gate Check**:
- ✅ Server-Side First: Works without JavaScript
- ✅ Static Content: File storage verified
- ✅ HTML/CSS First: JS-disabled test passed
- ✅ Educational Quality: Tests demonstrate patterns
- ✅ Progressive Enhancement: Degradation verified

### Phase 5: Documentation & Deployment

**Duration**: {X hours}

**Objectives**:
- Document code
- Update README
- Create deployment guide
- Deploy to production

**Tasks**:
1. Add code comments
2. Update README.md
3. Document file storage format
4. Create quickstart guide
5. Build for production
6. Deploy application

**Deliverables**:
- [ ] Code documented
- [ ] README updated
- [ ] Quickstart guide created
- [ ] Production build successful
- [ ] Application deployed

**Constitution Gate Check**:
- ✅ Server-Side First: Documented approach
- ✅ Static Content: Storage format documented
- ✅ HTML/CSS First: Approach documented
- ✅ Educational Quality: Clear documentation
- ✅ Progressive Enhancement: Enhancement layers documented

## File Structure

### New Files

```text
src/
├── handlers/
│   └── {feature}.rs          # NEW: Feature handler
├── models/
│   └── {entity}.rs           # NEW: Data model
templates/
├── {page}.html               # NEW: Main page template
└── partials/
    └── {component}.html      # NEW: Component partial
data/
└── {entity}/                 # NEW: Data storage directory
```

### Modified Files

```text
src/
├── main.rs                   # MODIFIED: App state (if needed)
└── routes.rs                 # MODIFIED: Add new routes
Cargo.toml                    # MODIFIED: Add dependencies
```

## Code Patterns

### Axum Handler Pattern

```rust
use axum::{
    extract::{State, Form},
    response::Html,
};
use askama::Template;

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate {
    field: String,
}

pub async fn handle_get(
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    let template = PageTemplate {
        field: "value".to_string(),
    };
    Ok(Html(template.render()?))
}
```

### File Storage Pattern

```rust
use chrono::Utc;
use std::fs;

fn save_to_file(data: &Entity) -> Result<(), Error> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%S-%3fZ");
    let filename = format!("data/entity/{}.json", timestamp);
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&filename, json)?;
    Ok(())
}
```

### Askama Template Pattern

```html
{% extends "base.html" %}

{% block title %}Page Title{% endblock %}

{% block content %}
<main class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold mb-6">Heading</h1>
  
  {% include "partials/component.html" %}
</main>
{% endblock %}
```

### HTMX Enhancement Pattern (if needed)

```html
<!-- Base functionality without JavaScript -->
<form method="post" action="/endpoint">
  <input type="text" name="field" required>
  <button type="submit">Submit</button>
</form>

<!-- Enhanced with HTMX (progressive enhancement) -->
<form method="post" action="/endpoint"
      hx-post="/endpoint"
      hx-target="#result"
      hx-swap="outerHTML">
  <input type="text" name="field" required>
  <button type="submit">Submit</button>
</form>
<div id="result"></div>
```

## Testing Approach

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        // Test validation logic
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_handler() {
    // Test handler request/response
}
```

### Manual Testing Checklist

- [ ] Form submission works
- [ ] Validation errors display correctly
- [ ] Success message appears
- [ ] Data saved to correct location
- [ ] Works with JavaScript disabled
- [ ] Keyboard navigation functional
- [ ] Screen reader announces correctly
- [ ] Mobile layout responsive

## Deployment Steps

### Build

```bash
# Build CSS
tailwindcss -i static/css/input.css -o static/css/output.css --minify

# Build Rust application
cargo build --release
```

### Run

```bash
./target/release/ppst
```

### Environment

- No environment variables needed (per constitution)
- Configuration in code or version-controlled files

## Review Checkpoints

### Phase 1 Review
- [ ] Structure matches plan
- [ ] Dependencies appropriate
- [ ] Base templates created

### Phase 2 Review
- [ ] Handler logic clear and educational
- [ ] File storage working correctly
- [ ] Error handling comprehensive

### Phase 3 Review
- [ ] Templates semantic and accessible
- [ ] Styling responsive and clean
- [ ] Progressive enhancement verified

### Phase 4 Review
- [ ] All tests passing
- [ ] Works without JavaScript
- [ ] Accessibility verified

### Phase 5 Review
- [ ] Documentation complete
- [ ] Deployment successful
- [ ] Constitution compliance verified

## Dependencies

### Rust Crates

```toml
[dependencies]
axum = "{version}"
tokio = { version = "{version}", features = ["full"] }
askama = "{version}"
serde = { version = "{version}", features = ["derive"] }
serde_json = "{version}"
chrono = "{version}"
```

### Build Dependencies

- Rust toolchain (stable)
- Tailwind CSS CLI standalone

## Success Criteria

- [ ] All specification requirements met
- [ ] All constitution gates passed
- [ ] Tests passing
- [ ] Documentation complete
- [ ] Production deployment successful
- [ ] Educational code quality verified

---

**Notes**:
- Follow constitution principles strictly
- Each phase must pass constitution gate check
- Maintain educational code quality throughout
- Document as you implement

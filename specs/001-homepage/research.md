# Research: Academy Homepage Implementation

**Feature**: 001-homepage
**Date**: 2025-10-30
**Status**: Complete

## Overview

This document consolidates research findings for implementing the PPST Academy homepage using Rust/Axum/Askama/HTMX stack with progressive enhancement principles.

## Technology Stack Research

### 1. Axum Web Framework

**Decision**: Use Axum as the web framework

**Rationale**:
- Mandated by project constitution
- Built on Tokio for excellent async performance
- Type-safe routing with compile-time guarantees
- Excellent middleware/tower integration
- Strong community support and active development

**Best Practices**:
- Use `Router::new()` for route composition
- Leverage extractors for request data (`Form`, `Query`, `State`)
- Use `ServiceBuilder` for middleware chains
- Structure handlers as async functions returning `impl IntoResponse`
- Use `tower-http` for serving static files

**Resources**:
- Official Axum examples: https://github.com/tokio-rs/axum/tree/main/examples
- Axum docs: https://docs.rs/axum/latest/axum/

### 2. Askama Templating

**Decision**: Use Askama for HTML templating

**Rationale**:
- Mandated by project constitution
- Compile-time template checking (catches errors before runtime)
- Zero-cost abstractions with Rust's type system
- Jinja2-like syntax (familiar to many developers)
- Perfect for server-side rendering

**Best Practices**:
- Create base template with `{% block %}` for inheritance
- Use `#[derive(Template)]` on structs for type-safe rendering
- Store templates in `templates/` directory
- Use partials for reusable components (header, footer, forms)
- Leverage template inheritance to avoid duplication

**Template Structure Pattern**:
```rust
#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    academy_name: String,
    mission: String,
    programs: Vec<Program>,
    instructors: Vec<Instructor>,
}
```

**Resources**:
- Askama book: https://djc.github.io/askama/
- Template syntax reference: https://djc.github.io/askama/template_syntax.html

### 3. HTMX Integration

**Decision**: Use HTMX for progressive enhancement

**Rationale**:
- Mandated by project constitution
- Enables rich interactions without complex JavaScript
- Works via HTML attributes (declarative)
- Gracefully degrades when JS disabled
- Perfect for server-side-first architecture

**Best Practices for This Feature**:
- Form submission: Use `hx-post` for AJAX submission, fallback to regular POST
- Response handling: Return HTML fragments or full pages based on `HX-Request` header
- Error handling: Return appropriate HTML with error messages
- Loading states: Use `hx-indicator` for user feedback during requests
- Keep HTMX optional - all functionality must work without it

**Implementation Pattern**:
```html
<!-- Progressive enhancement: works with or without HTMX -->
<form action="/contact" method="post" hx-post="/contact" hx-target="#contact-result">
    <input type="email" name="email" required>
    <button type="submit">Submit</button>
</form>
<div id="contact-result"></div>
```

**Server-side Pattern** (Axum):
```rust
async fn handle_contact(
    headers: HeaderMap,
    Form(contact): Form<ContactForm>
) -> impl IntoResponse {
    // Process form
    let is_htmx = headers.get("HX-Request").is_some();

    if is_htmx {
        // Return partial HTML for HTMX
        Html("<div class='success'>Thanks for contacting us!</div>")
    } else {
        // Return full page redirect for non-HTMX
        Redirect::to("/").into_response()
    }
}
```

**Resources**:
- HTMX docs: https://htmx.org/docs/
- Examples: https://htmx.org/examples/

### 4. Tailwind CSS

**Decision**: Use Tailwind CSS for styling

**Rationale**:
- Mandated by project constitution
- Utility-first approach enables rapid development
- Excellent responsive design support
- Small production bundle (unused classes purged)
- No custom CSS needed for most use cases

**Best Practices**:
- Use Tailwind CLI for CSS generation
- Configure `tailwind.config.js` to scan template files
- Use responsive prefixes (`md:`, `lg:`) for device adaptation
- Leverage semantic color names in config for brand consistency
- Use `@apply` sparingly - prefer utility classes

**Configuration**:
```js
// tailwind.config.js
module.exports = {
  content: ["./src/templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        'academy-blue': '#...',
        'academy-gold': '#...',
      }
    }
  }
}
```

**Resources**:
- Tailwind docs: https://tailwindcss.com/docs
- Responsive design: https://tailwindcss.com/docs/responsive-design

### 5. File-Based Storage

**Decision**: Store contact submissions as JSON files

**Rationale**:
- Mandated by constitution (no databases)
- Simple, transparent storage
- Easy to review submissions manually
- Version-controllable (if desired)
- Sufficient for low-volume promotional site

**Best Practices**:
- Use `serde_json` for serialization
- Store each submission as separate file with timestamp-based name
- Use atomic file writes to prevent corruption
- Include error handling for file I/O operations
- Create directory structure if it doesn't exist

**Implementation Pattern**:
```rust
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct ContactSubmission {
    timestamp: String,
    name: String,
    email: String,
    subject: Option<String>,
    message: String,
}

async fn save_submission(submission: ContactSubmission) -> Result<()> {
    let filename = format!("data/contacts/{}.json", submission.timestamp);
    let json = serde_json::to_string_pretty(&submission)?;
    fs::write(filename, json)?;
    Ok(())
}
```

**Resources**:
- serde_json: https://docs.rs/serde_json/latest/serde_json/
- std::fs: https://doc.rust-lang.org/std/fs/

## Form Validation Research

### Server-Side Validation

**Decision**: Implement validation in Axum handlers with validation library

**Approaches Evaluated**:
1. **Manual validation** - Simple for basic cases
2. **validator crate** - Derive-based validation attributes
3. **garde crate** - Alternative validation library

**Selected**: **Manual validation** for this simple form

**Rationale**:
- Only 3-4 fields to validate
- Requirements are straightforward (required fields, email format)
- Avoids additional dependency for simple needs
- Educational value in showing explicit validation logic

**Implementation Pattern**:
```rust
fn validate_contact_form(form: &ContactForm) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if form.name.trim().is_empty() {
        errors.push("Name is required".to_string());
    }

    if !is_valid_email(&form.email) {
        errors.push("Valid email is required".to_string());
    }

    if form.message.trim().is_empty() {
        errors.push("Message is required".to_string());
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
```

### Client-Side Validation

**Decision**: HTML5 validation attributes + CSS feedback

**Implementation**:
- Use `required` attribute on mandatory fields
- Use `type="email"` for email validation
- Use `:invalid` and `:valid` CSS pseudo-classes for visual feedback
- No JavaScript required

**Pattern**:
```html
<input
    type="email"
    name="email"
    required
    class="border focus:ring-2 invalid:border-red-500"
    placeholder="your@email.com"
>
```

## Responsive Design Research

### Breakpoint Strategy

**Decision**: Use Tailwind's default breakpoints

**Breakpoints**:
- `sm`: 640px (small tablets)
- `md`: 768px (tablets)
- `lg`: 1024px (laptops)
- `xl`: 1280px (desktops)
- `2xl`: 1536px (large screens)

**Layout Approach**:
- **Mobile-first**: Base styles for mobile, use prefixes for larger screens
- **Single-column on mobile**: Stack sections vertically
- **Multi-column on desktop**: Use CSS Grid for instructor cards, program listings
- **Responsive typography**: Use Tailwind's responsive text utilities

**Example**:
```html
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    <!-- Cards automatically adjust to screen size -->
</div>
```

## Accessibility Research (WCAG 2.1 Level AA)

### Key Requirements

**Decision**: Implement the following accessibility features

**Mandatory Elements**:
1. **Semantic HTML**: Use `<header>`, `<nav>`, `<main>`, `<section>`, `<footer>`
2. **Form labels**: All inputs must have associated `<label>` elements
3. **Focus indicators**: Visible focus states for keyboard navigation
4. **Color contrast**: Text must meet 4.5:1 ratio for normal text
5. **Alt text**: All images (instructor photos, logo) need descriptive alt attributes
6. **ARIA labels**: Use when semantic HTML insufficient

**Implementation**:
```html
<!-- Semantic HTML -->
<main id="main-content">
    <section aria-labelledby="mission-heading">
        <h2 id="mission-heading">Our Mission</h2>
        <!-- content -->
    </section>
</main>

<!-- Form accessibility -->
<label for="email" class="block mb-2">
    Email Address <span class="text-red-500">*</span>
</label>
<input
    id="email"
    type="email"
    name="email"
    required
    aria-required="true"
    aria-describedby="email-hint"
>
<span id="email-hint" class="text-sm text-gray-600">
    We'll never share your email
</span>
```

### Keyboard Navigation

**Decision**: Ensure full keyboard accessibility

**Requirements**:
- Tab order follows logical reading order
- Skip links for main content
- Focus trap in modal dialogs (if any)
- Enter key submits forms
- Escape key closes modals (if any)

**Pattern**:
```html
<!-- Skip link (hidden visually, available to screen readers) -->
<a href="#main-content" class="sr-only focus:not-sr-only">
    Skip to main content
</a>
```

**Resources**:
- WCAG 2.1: https://www.w3.org/WAI/WCAG21/quickref/
- WebAIM checklist: https://webaim.org/standards/wcag/checklist

## Performance Optimization Research

### Static Asset Handling

**Decision**: Use `tower-http` for serving static files

**Pattern**:
```rust
use tower_http::services::ServeDir;

let app = Router::new()
    .route("/", get(homepage))
    .nest_service("/static", ServeDir::new("static"));
```

**Optimizations**:
- Minify Tailwind CSS in production (via CLI flag)
- Use CDN for HTMX if preferred (fallback to local copy)
- Compress static assets with gzip/brotli (tower-http middleware)

### Template Rendering Performance

**Decision**: Askama compiles templates at build time

**Benefits**:
- Zero runtime template parsing overhead
- Templates are part of the compiled binary
- Type checking catches errors at compile time

**Note**: No runtime optimization needed - Askama handles this

### Caching Strategy

**Decision**: Minimal caching for now

**Rationale**:
- Static content changes infrequently
- Dynamic elements (form) shouldn't be cached
- Can add HTTP caching headers later if needed

**Future Consideration**:
```rust
use tower_http::set_header::SetResponseHeaderLayer;

// Add cache headers for static assets
.layer(SetResponseHeaderLayer::if_not_present(
    header::CACHE_CONTROL,
    HeaderValue::from_static("public, max-age=3600"),
))
```

## Alternatives Considered

### Template Engines

**Alternatives**: Tera, Handlebars, minijinja

**Why Askama**:
- Compile-time safety (mandated by constitution)
- Performance (no runtime parsing)
- Type integration with Rust

### CSS Frameworks

**Alternatives**: Bootstrap, Bulma, custom CSS

**Why Tailwind**:
- Mandated by constitution
- Utility-first approach matches server-side-first philosophy
- Excellent purging (small bundles)

### Storage Solutions

**Alternatives**: SQLite, PostgreSQL, cloud storage

**Why File Storage**:
- Mandated by constitution (no databases)
- Simple for low volume
- Easy to implement and maintain

## Conclusion

All research findings support the technology choices mandated by the project constitution. Implementation patterns are well-established, with strong community support and documentation for each component. The stack (Rust/Axum/Askama/HTMX/Tailwind) works cohesively to enable progressive enhancement and server-side-first architecture.

**No NEEDS CLARIFICATION items remain** - all technical decisions are either mandated by constitution or have clear best practices from research.

**Ready to proceed to Phase 1: Design & Contracts**

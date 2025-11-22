# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **server-side first educational website** for PPST Academy using Rust, Axum, and Modern Vanilla CSS. The project emphasizes minimal dependencies, zero build complexity, and centralized CSS architecture.

**Philosophy**: Server-rendered HTML, progressive enhancement with HTMX, no database (file-based JSON storage), and pure CSS without preprocessors.

## Essential Commands

### Development

```bash
# Run development server (http://localhost:3000)
cargo run

# Auto-reload on changes (requires cargo-watch)
cargo watch -x run

# Production build
cargo build --release
./target/release/ppst-academy
```

### Testing & Quality

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Lint code
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Version Control (Jujutsu)

```bash
# View status and changes
jj status
jj diff
jj log

# Commit changes
jj describe -m "type: message"
jj new

# Sync with remote
jj git fetch
jj branch set main
jj git push
```

**Commit format**: Use conventional commits (`feat:`, `fix:`, `style:`, `refactor:`, `docs:`, `test:`, `chore:`)

## Architecture

### Request Flow

```
Browser → routes.rs → handlers/ → storage/models → templates/ → HTML Response
```

### Module Structure

- **src/main.rs** - Application entry point, server setup
- **src/routes.rs** - Route definitions (GET /, POST /contact)
- **src/error.rs** - Centralized error handling with thiserror (type-safe, automatic conversions)
- **src/handlers/** - Request handlers with dual-mode response (HTMX fragments + standard redirects)
- **src/models/** - Data structures and validation
- **src/storage/** - File-based JSON storage repository
- **templates/** - Askama HTML templates (Jinja2-style)
- **static/css/** - 7-1 CSS architecture

### CSS Architecture (7-1 + BEM + Cascade Layers)

**Layer Priority** (defined in [static/css/main.css](static/css/main.css:19)):

```css
@layer reset, base, layout, components, pages, vendors;
```

**Directory Structure**:

- `abstracts/` - CSS variables (design tokens)
- `base/` - Reset, typography
- `layout/` - Container, grid, header, footer
- `components/` - Button, card, form, navigation, avatar, spinner, alert
- `pages/` - Page-specific styles
- `themes/` - Theme variations
- `vendors/` - HTMX styles

**BEM Naming**:

- Block: `.card`, `.button`, `.form`
- Element: `.card__title`, `.button__icon`, `.form__input`
- Modifier: `.card--hoverable`, `.button--primary`, `.form--large`

### Dual-Mode Response Pattern

All handlers support **progressive enhancement** via HTMX, ensuring the application works with or without JavaScript:

#### Response Modes

1. **HTMX Request** (detected via `hx-request` header):
   - Returns HTML fragments for dynamic updates
   - Status codes: 200 OK, 400 Bad Request, 429 Too Many Requests
   - Rendered via Askama templates in `templates/partials/`
   - Enables smooth UX without full page reloads

2. **Standard Request** (JavaScript disabled or direct form submission):
   - Returns redirects or full HTML pages
   - Falls back to traditional form POST behavior
   - Ensures accessibility and baseline functionality

#### Implementation Pattern

Example in [src/handlers/contact.rs](src/handlers/contact.rs:166):

```rust
// Helper function to detect HTMX requests
fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers.get("hx-request").is_some()
}

// Dual-mode response
if is_htmx_request(&headers) {
    // HTMX: Return HTML fragment
    let template = ContactSuccessTemplate {};
    Html(template.render()?).into_response()
} else {
    // Standard: Return redirect
    Redirect::to("/?success=true").into_response()
}
```

#### Why This Matters

- **Progressive Enhancement**: Core functionality works without JavaScript
- **Accessibility**: Screen readers and keyboard navigation fully supported
- **Resilience**: Application degrades gracefully on older browsers
- **SEO**: Search engines can crawl standard HTML responses
- **Single Endpoint**: No need for separate API + page routes

### Rate Limiting

In-memory rate limiter using lazy_static HashMap ([src/handlers/contact.rs](src/handlers/contact.rs:28-51)):

- 30-second window per IP address
- Shared state: `Arc<Mutex<HashMap<String, Instant>>>`

### Data Storage

File-based JSON storage in `data/contacts/`:

- Pattern: `YYYY-MM-DDTHH-MM-SS-mmmZ.json`
- Auto-creates directories via [src/storage/file_store.rs](src/storage/file_store.rs:10-13)
- Pretty-printed JSON for human readability

### Error Handling

Centralized error handling using **thiserror** ([src/error.rs](src/error.rs)):

#### Design Philosophy

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to render template: {0}")]
    Template(#[from] askama::Error),

    #[error("Storage error: {0}")]
    Storage(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    // ... more variants
}
```

**Key Features:**

- **Type-safe error handling**: Each error variant is strongly typed
- **Automatic conversions**: `#[from]` attribute enables `?` operator to auto-convert errors
- **HTTP response integration**: `IntoResponse` trait converts errors to user-friendly HTTP responses
- **Structured logging**: All errors logged via `tracing::error!` before conversion
- **Convenience alias**: `Result<T>` type alias for cleaner function signatures

**Usage Pattern:**

```rust
pub fn save_contact_inquiry(inquiry: &ContactInquiry) -> Result<()> {
    let json = serde_json::to_string_pretty(inquiry)?;  // Auto-converts to AppError
    file.write_all(json.as_bytes())?;                    // Auto-converts to AppError
    Ok(())
}
```

**Benefits:**

- No manual error wrapping with `.map_err()`
- Compile-time verification of error handling
- User-friendly error messages in production
- Developer-friendly error messages in logs

## Development Patterns

### Adding Routes

1. Define route in [src/routes.rs](src/routes.rs):

```rust
.route("/new-route", get(handlers::new_handler::get_page))
```

2. Create handler in `src/handlers/new_handler.rs`:

```rust
pub async fn get_page() -> impl IntoResponse {
    // Handler logic
}
```

3. Add handler module to `src/handlers/mod.rs`

### Adding CSS Components

1. Create file: `static/css/components/_new-component.css`
2. Use BEM naming convention
3. Wrap styles in cascade layer:

```css
@layer components {
  .component { /* styles */ }
  .component__element { /* styles */ }
  .component--modifier { /* styles */ }
}
```

4. Import in [static/css/main.css](static/css/main.css)
5. Use BEM classes in templates

### Editing Design Tokens

Edit [static/css/abstracts/_variables.css](static/css/abstracts/_variables.css):

```css
:root {
  --color-brand: #2563eb;
  --font-size-xl: 1.25rem;
  --space-section-y: clamp(4rem, 6vw, 6rem);
  --radius-lg: 0.75rem;
  --transition-base: 200ms ease-in-out;
}
```

No build step needed - CSS changes are immediately reflected on refresh.

### Askama Templates

Templates use Jinja2-like syntax ([Askama docs](https://docs.rs/askama)):

- Variables: `{{ variable }}`
- Conditionals: `{% if condition %} ... {% endif %}`
- Loops: `{% for item in items %} ... {% endfor %}`
- Includes: `{% include "partials/header.html" %}`

Templates are **type-checked at compile time**.

## Key Technologies

### Backend

- **Axum 0.8** - Web framework
- **Tokio** - Async runtime
- **Tower-HTTP** - Static file serving + Gzip compression
- **Askama 0.14** - Type-safe server-side templates
- **Serde** - JSON serialization
- **Chrono** - Date/time handling
- **Regex** - Email validation

### Frontend

- **Vanilla CSS** - No preprocessors (Sass/Less/PostCSS)
- **CSS Custom Properties** - Design tokens
- **CSS Cascade Layers** - Explicit style priority
- **HTMX** - Progressive enhancement (local file, no CDN)

### Constraints (Not Used)

- ❌ Node.js / npm / yarn / pnpm
- ❌ CSS preprocessors
- ❌ JavaScript frameworks
- ❌ Build tools (webpack, vite, rollup)
- ❌ Databases (SQL, NoSQL)
- ❌ Tailwind CSS or other CSS frameworks

## Port Configuration

Server binds to `localhost:3000` by default. Override via environment variables:

```bash
PORT=8080 cargo run
# OR
PPST_PORT=8080 cargo run
```

Configuration in [src/main.rs](src/main.rs:30-35).

## Logging

Using `tracing` + `tracing-subscriber`:

- Default: `ppst_academy=debug,tower_http=debug`
- Override: `RUST_LOG=ppst_academy=info cargo run`

## Testing Strategy

- Unit tests in same files as source code (`#[cfg(test)]` modules)
- Test storage layer functions in [src/storage/file_store.rs](src/storage/file_store.rs:30-52)
- Manual testing checklist in README.md

## File References

When working with code:

- Routes: [src/routes.rs](src/routes.rs)
- Main entry: [src/main.rs](src/main.rs)
- Contact handler: [src/handlers/contact.rs](src/handlers/contact.rs)
- Contact model: [src/models/contact.rs](src/models/contact.rs)
- File storage: [src/storage/file_store.rs](src/storage/file_store.rs)
- CSS entry: [static/css/main.css](static/css/main.css)
- Design tokens: [static/css/abstracts/_variables.css](static/css/abstracts/_variables.css)
- Base template: [templates/base.html](templates/base.html)

## Important Notes

- **Rust Edition 2024**: Code uses latest stable Rust features (let-else patterns, if-let guards)
- **No CDN dependencies**: HTMX is served from `static/js/htmx.min.js`
- **Single CSS file**: All CSS imports bundled via browser's native `@import`
- **Type safety**: Askama templates are validated at compile-time
- **Email validation**: Simplified RFC 5322 regex in [src/models/contact.rs](src/models/contact.rs:109-116)

# PPST Academy Website

Modern educational website for academy promotion (학원 홍보 홈페이지)

## Table of Contents

- [Overview](#overview)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
- [Development Guide](#development-guide)
- [Version Control](#version-control)
- [Testing](#testing)
- [API Reference](#api-reference)

---

## Overview

A **lightweight, server-side first** educational website demonstrating modern web development with minimal dependencies.

### Key Features

- **Server-Side Rendering** - Rust + Axum + Askama templates
- **Modern Vanilla CSS** - 7-1 architecture + BEM methodology + CSS Cascade Layers
- **Progressive Enhancement** - HTMX for form interactions
- **File-Based Storage** - JSON files, no database required
- **Zero Build Tools** - No npm, webpack, or CSS preprocessors

### Design Philosophy

- ✅ **Server-Side First** - HTML rendered on server, minimal client JavaScript
- ✅ **Centralized CSS** - Modular architecture with clear organization
- ✅ **Zero Complexity** - No build steps, preprocessors, or bundlers
- ✅ **Maintainable** - BEM naming, explicit cascade layers, clear file structure
- ✅ **Lightweight** - 2.8MB source code, single CSS file served

---

## Technology Stack

### Backend (Rust)

**Core Framework:**
- **Axum 0.7** - Web framework
- **Tokio 1.x** - Async runtime
- **Tower 0.4** - Middleware framework
- **Tower-HTTP 0.5** - Static file serving + Gzip compression

**Templating:**
- **Askama 0.12** - Server-side HTML templates (Jinja2-style)
- **Askama-Axum 0.4** - Axum integration

**Data & Utilities:**
- **Serde 1.0** + **Serde JSON 1.0** - JSON serialization
- **Chrono 0.4** - Date/time handling
- **Regex 1.10** - Email validation
- **Lazy Static 1.4** - Global static variables

### Frontend

**Styling:**
- **Modern Vanilla CSS** - No preprocessors (Sass/Less/PostCSS)
- **CSS Custom Properties** - Design tokens via CSS variables
- **CSS Cascade Layers** - Explicit style priority with `@layer`
- **7-1 Architecture** - Organized file structure pattern
- **BEM Methodology** - Consistent class naming convention

**Interactivity:**
- **HTMX** - Server communication (local file, no CDN)
  - Progressive enhancement
  - Form submission (AJAX)
  - Partial HTML updates

### Data Storage

- **File-Based JSON** - No database required
- **Pattern**: `data/contacts/2024-11-06T12-30-45-123Z.json`
- **Timestamped filenames** - Automatic organization

### Tools

- **Cargo** - Rust package manager & build system
- **Jujutsu (jj)** - Git-compatible version control

### Constraints

**❌ Not Used:**
- Node.js / npm / yarn / pnpm
- CSS preprocessors (Sass, Less, PostCSS)
- CSS frameworks (Tailwind, Bootstrap)
- JavaScript bundlers (webpack, vite, rollup)
- Databases (SQL, NoSQL)
- CDN dependencies

---

## Project Structure

```text
ppst/
├── src/                     # Rust source code
│   ├── main.rs              # Application entry point
│   ├── routes.rs            # Route definitions
│   ├── handlers/            # Request handlers
│   │   ├── homepage.rs      # Homepage logic
│   │   └── contact.rs       # Contact form logic
│   ├── models/              # Data structures
│   │   ├── academy.rs       # Academy info model
│   │   └── contact.rs       # Contact form model
│   └── storage/             # File storage
│       └── file_store.rs    # JSON file operations
│
├── templates/               # Askama HTML templates (BEM classes)
│   ├── base.html            # Base layout
│   ├── homepage.html        # Homepage template
│   └── partials/            # Reusable components
│       ├── header.html
│       ├── mission.html
│       ├── programs.html
│       ├── instructors.html
│       ├── contact_form.html
│       ├── contact_success.html
│       └── contact_error.html
│
├── static/                  # Static assets (served by Tower-HTTP)
│   ├── css/                 # 🎨 Modern Vanilla CSS (7-1 pattern)
│   │   ├── abstracts/       # Variables (_variables.css, _mixins.css)
│   │   ├── base/            # Reset & typography
│   │   ├── layout/          # Container, header, footer, grid
│   │   ├── components/      # Button, card, form, navigation, etc.
│   │   ├── pages/           # Page-specific styles (_homepage.css)
│   │   ├── themes/          # Theme variations
│   │   ├── vendors/         # Third-party styles (_htmx.css)
│   │   └── main.css         # Main entry (imports all modules)
│   └── js/
│       └── htmx.min.js      # HTMX library (local copy)
│
├── data/
│   └── contacts/            # Contact form submissions (JSON files)
│
├── Cargo.toml               # Rust dependencies
├── Cargo.lock               # Dependency lock file
├── .gitignore               # Git ignore rules
└── README.md                # This file
```

### CSS Architecture Details

**7-1 Pattern Structure:**

```text
static/css/
├── abstracts/      # Variables, design tokens
├── base/           # Reset, typography, base styles
├── layout/         # Layout components (header, footer, grid)
├── components/     # Reusable UI components (buttons, cards, forms)
├── pages/          # Page-specific styles
├── themes/         # Theme variations
├── vendors/        # Third-party library styles
└── main.css        # Main entry point that imports all
```

**CSS Cascade Layers:**

Explicit style priority defined in [main.css](static/css/main.css):

```css
@layer reset, base, layout, components, pages, vendors;
```

This eliminates specificity wars and ensures predictable styling.

**BEM Naming Convention:**

- **Block**: `.card`, `.button`, `.form`
- **Element**: `.card__title`, `.button__icon`, `.form__input`
- **Modifier**: `.card--hoverable`, `.button--primary`, `.form--large`

---

## Architecture

### Layered Architecture (MVC + Repository Pattern)

**Patterns:**
- MVC architecture with repository abstraction
- Server-side rendering (SSR)
- Progressive enhancement
- File-based storage

**Request Flow:**
```
Browser → routes.rs → handlers → storage/models → templates → HTML response
```

**Component Mapping:**
- **Presentation**: Axum handlers + Askama templates
- **Styling**: Modular CSS (7-1 + BEM + Cascade Layers)
- **Domain**: Rust models (data structures)
- **Data Access**: File-based storage repository

**Benefits:**
- Clear separation of concerns
- Testable (mock storage layer)
- Maintainable CSS with BEM
- Predictable cascade with `@layer`
- Easy to replace any layer independently

### CSS Layer Order

1. **reset** - CSS reset for consistent baseline
2. **base** - Typography and base element styles
3. **layout** - Layout components (grid, container, header, footer)
4. **components** - Reusable UI components
5. **pages** - Page-specific styles
6. **vendors** - Third-party library styles

---

## Getting Started

### Prerequisites

- **Rust** (latest stable) - [Install](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Quick Start

```bash
# Clone repository
git clone git@github.com:hakchin/ppst.git
cd ppst

# Run server
cargo run

# Visit http://localhost:3000
```

### Build for Production

```bash
cargo build --release
./target/release/ppst-academy
```

---

## Development Guide

### Basic Workflow

```bash
# Run server
cargo run

# With auto-reload (optional)
cargo install cargo-watch  # Install once
cargo watch -x run         # Auto-restart on Rust changes
```

### CSS Development

**Editing Styles:**

1. Edit any CSS file in `static/css/` directory
2. Refresh browser (no build step needed!)
3. Pure CSS - no compilation required

**Adding New Components:**

1. Create file: `static/css/components/_component-name.css`
2. Use BEM naming: `.component`, `.component__element`, `.component--modifier`
3. Wrap in layer: `@layer components { ... }`
4. Import in `static/css/main.css`
5. Use classes in HTML templates

**Design Tokens:**

Edit [static/css/abstracts/_variables.css](static/css/abstracts/_variables.css):

```css
:root {
  /* Colors */
  --color-brand: #2563eb;
  --color-text: #1f2937;

  /* Typography */
  --font-size-xl: 1.25rem;
  --font-weight-semibold: 600;

  /* Spacing */
  --space-section-y: clamp(4rem, 6vw, 6rem);
  --gap-grid: 2.5rem;

  /* Border Radius */
  --radius-lg: 0.75rem;

  /* Transitions */
  --transition-base: 200ms ease-in-out;
}
```

### Adding Routes & Handlers

1. Define route in [src/routes.rs](src/routes.rs)
2. Create handler in `src/handlers/`
3. Create template in `templates/`
4. Add CSS if needed in `static/css/pages/`

---

## Version Control

This project uses **Jujutsu (jj)**, a Git-compatible VCS.

### Essential Commands

```bash
# View status
jj status                    # Current working state
jj diff                      # Uncommitted changes
jj log                       # Commit history (@ = current)

# Make commits
jj describe -m "type: message"   # Describe changes
jj new                           # Create new working change

# Sync with remote
jj git fetch                     # Fetch from remote
jj branch set main               # Update branch pointer
jj git push                      # Push to remote

# Undo changes
jj restore <file>                # Restore specific file
jj restore --from @-             # Restore all changes
```

### Commit Message Format

**Conventional Commits**: `type: description`

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code restructuring
- `style:` CSS/formatting changes
- `docs:` Documentation updates
- `test:` Test additions
- `chore:` Maintenance tasks

**Examples:**
```bash
jj describe -m "feat: add user profile page"
jj describe -m "fix: correct email validation regex"
jj describe -m "style: update hero section gradient"
jj describe -m "refactor: convert to BEM methodology"
```

### Typical Workflow

```bash
# 1. Make changes
# 2. Test
cargo test && cargo run

# 3. Commit
jj describe -m "feat: add new feature"
jj new

# 4. Push
jj branch set main
jj git push
```

---

## Testing

### Run Tests

```bash
cargo test                    # All tests
cargo test -- --nocapture     # With output
```

### Code Quality

```bash
cargo clippy                  # Linter
cargo fmt                     # Auto-format code
cargo fmt -- --check          # Check formatting
```

### Manual Testing

- [ ] Server starts without errors
- [ ] Homepage loads at `http://localhost:3000`
- [ ] CSS styles render correctly (BEM classes)
- [ ] Contact form validates input
- [ ] Success/error messages display properly
- [ ] Data saves to `data/contacts/*.json`
- [ ] Responsive design works on mobile/tablet/desktop
- [ ] HTMX loading indicator appears during requests

---

## API Reference

### Endpoints

| Method | Path       | Description                                | Response |
|--------|------------|--------------------------------------------|----------|
| GET    | `/`        | Homepage with academy info & contact form  | HTML     |
| POST   | `/contact` | Submit contact form                        | Partial HTML (HTMX) |

### Contact Form Submission

**Request:**
```http
POST /contact
Content-Type: application/x-www-form-urlencoded

name=홍길동&email=hong@example.com&subject=문의&message=학원에 대해 궁금합니다
```

**Success Response:**
```html
<div class="alert alert--success">
  <p class="alert__text">Thank you for contacting PPST Academy...</p>
</div>
```

**Error Response:**
```html
<div class="alert alert--error">
  <p class="alert__title">Please correct the following errors:</p>
  <ul>
    <li class="alert__text">Email is required</li>
  </ul>
</div>
```

### Data Storage Format

**File Pattern**: `data/contacts/2024-11-06T12-30-45-123Z.json`

**JSON Structure**:
```json
{
  "name": "홍길동",
  "email": "hong@example.com",
  "subject": "문의",
  "message": "학원에 대해 궁금한 점이 있습니다.",
  "timestamp": "2024-11-06T12:30:45.123Z"
}
```

---

## Quick Reference

### Daily Commands

```bash
# Development
cargo run                         # Start server

# Version Control
jj status                         # Check status
jj describe -m "type: message"   # Commit changes
jj new                            # New working change
jj git push                       # Push to remote

# Code Quality
cargo test                        # Run tests
cargo clippy                      # Lint code
cargo fmt                         # Format code
```

### Key Directories

| Path | Purpose |
|------|---------|
| `src/` | Rust source code |
| `templates/` | HTML templates (Askama) |
| `static/css/` | CSS architecture (7-1 + BEM) |
| `static/js/` | JavaScript (HTMX only) |
| `data/contacts/` | Form submissions (JSON) |

### Important Files

| File | Description |
|------|-------------|
| [src/main.rs](src/main.rs) | Application entry point |
| [src/routes.rs](src/routes.rs) | Route definitions |
| [static/css/main.css](static/css/main.css) | CSS entry point |
| [static/css/abstracts/_variables.css](static/css/abstracts/_variables.css) | Design tokens |
| [templates/base.html](templates/base.html) | Base HTML layout |
| [Cargo.toml](Cargo.toml) | Rust dependencies |

### Project Stats

- **Source Code Size**: 2.8MB
- **Backend Language**: Rust (100%)
- **Frontend Styling**: Vanilla CSS (100%)
- **JavaScript**: 1 file (HTMX only)
- **Dependencies**: 9 Rust crates
- **CSS Files**: 19 files (7-1 pattern)
- **Build Tools**: Cargo only

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Askama Templates](https://docs.rs/askama/latest/askama/)
- [HTMX Documentation](https://htmx.org/)
- [BEM Methodology](https://getbem.com/)
- [CSS Cascade Layers](https://developer.mozilla.org/en-US/docs/Web/CSS/@layer)
- [7-1 Pattern](https://sass-guidelin.es/#the-7-1-pattern)
- [Jujutsu VCS](https://github.com/martinvonz/jj)

---

## Troubleshooting

### Port Already in Use

```bash
lsof -i :3000              # Find process using port 3000
kill -9 <PID>              # Kill the process
```

### Build Errors

```bash
cargo clean                # Clean build artifacts
cargo build                # Rebuild from scratch
```

### CSS Not Loading

1. Verify `static/css/main.css` exists
2. Check browser console for 404 errors
3. Hard refresh: `Cmd+Shift+R` (Mac) or `Ctrl+Shift+R` (Windows/Linux)
4. Verify server logs for static file serving errors

### Version Control Issues

**No changes to describe:**
- Make file changes before running `jj describe`

**Push rejected:**
```bash
jj git fetch               # Fetch latest changes
jj rebase -d main          # Rebase on main
jj git push                # Retry push
```

**Reset to remote (⚠️ discards local changes):**
```bash
jj git fetch
jj new main@origin
jj branch set main
```

---

## License

MIT

---

## Repository

**GitHub**: [https://github.com/hakchin/ppst](https://github.com/hakchin/ppst)

**Server**: `http://localhost:3000`

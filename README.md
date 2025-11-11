# PPST Academy Website

Modern educational website for academy promotion (학원 홍보 홈페이지)

## Table of Contents

- [Overview](#overview)
- [Core Principles](#core-principles)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Development Guide](#development-guide)
- [Version Control](#version-control)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)
- [API Reference](#api-reference)

---

## Overview

A **lightweight, server-side first** educational website built with Rust and HTMX. This learning project demonstrates modern web development with minimal JavaScript and centralized asset management.

### Key Features

- Server-rendered academy information (mission, programs, instructors)
- HTMX-enhanced contact form with progressive enhancement
- File-based JSON storage (no database required)
- Centralized CSS management via Tailwind CLI
- Simple, maintainable architecture without complex build tools

---

## Core Principles

### Design Philosophy

- ✅ **Server-Side First** - HTML rendered on server, minimal client JavaScript
- ✅ **Centralized Asset Management** - Single source of truth for CSS via Tailwind CLI
- ✅ **Zero Build Complexity** - No Node.js, npm, webpack, vite, or bundlers
- ✅ **Progressive Enhancement** - Core functionality works without JavaScript
- ✅ **File-Based Storage** - No database required, simple JSON files
- ✅ **Maintainable Architecture** - Clear separation of concerns, easy to modify

### Centralized CSS Management

**All styling is managed through a single centralized workflow:**

1. **Input Source**: `src/input.css` - Single entry point for all Tailwind directives
2. **Configuration**: `tailwind.config.js` - Centralized theme and plugin settings
3. **Output**: `static/css/tailwind.css` - Single compiled CSS file served to all pages
4. **Templates**: HTML templates use Tailwind utility classes consistently

**Benefits:**

- **Single Source of Truth**: All style changes go through one pipeline
- **Easy Maintenance**: Update `input.css` or config once, affects entire site
- **No CSS Duplication**: Tailwind purges unused styles automatically
- **Fast Rebuilds**: Watch mode instantly recompiles on changes
- **Version Control**: CSS changes tracked cleanly in git

### Technology Constraints

**Allowed:**

- Rust backend (Axum, Askama templates)
- HTMX for server communication (served locally from `static/js/`)
- Tailwind CSS via standalone CLI (no Node.js)
- File-based JSON storage

**Not Allowed:**

- Node.js, npm, yarn, pnpm
- JavaScript bundlers (webpack, vite, rollup, parcel)
- Database systems (SQL, NoSQL)
- CDN dependencies (all assets served locally)
- Excessive client-side JavaScript

---

## Technology Stack

### Backend

- **Language**: Rust (latest stable)
- **Web Framework**: Axum 0.7
- **Templating**: Askama 0.12
- **Async Runtime**: Tokio 1.0
- **Serialization**: Serde 1.0
- **Date/Time**: Chrono 0.4
- **Static Files**: Tower-HTTP 0.5

### Frontend

- **Styling**: Tailwind CSS (standalone CLI)
- **Interactivity**: HTMX (local static file)
- **Enhancement**: Alpine.js (minimal, only when necessary)

### Tools

- **CSS Build**: Tailwind CLI standalone binary
- **Version Control**: Jujutsu (Git-compatible)
- **Package Manager**: Cargo

---

## Project Structure

```text
ppst/
├── src/
│   ├── main.rs              # Application entry point
│   ├── routes.rs            # Route definitions
│   ├── input.css            # 🎨 Tailwind CSS input (centralized)
│   ├── handlers/            # Request handlers
│   │   ├── homepage.rs      # Homepage handler
│   │   └── contact.rs       # Contact form handler
│   ├── models/              # Data structures
│   │   ├── academy.rs       # Academy info model
│   │   └── contact.rs       # Contact form model
│   └── storage/             # File storage utilities
│       └── file_store.rs    # JSON file operations
├── templates/               # Askama templates
│   ├── base.html            # Base layout template
│   ├── homepage.html        # Homepage template
│   └── partials/            # Reusable components
│       ├── header.html
│       ├── mission.html
│       ├── programs.html
│       ├── instructors.html
│       ├── contact_form.html
│       ├── contact_success.html
│       └── contact_error.html
├── static/                  # Static assets (served by Tower-HTTP)
│   ├── css/
│   │   └── tailwind.css     # 🎨 Compiled CSS (generated, do not edit)
│   └── js/
│       └── htmx.min.js      # HTMX library (local copy)
├── data/
│   └── contacts/            # Contact submissions (JSON files)
├── Cargo.toml               # Rust dependencies
└── tailwind.config.js       # 🎨 Tailwind configuration (centralized)
```

### Key Files for CSS Management

- **`src/input.css`** - Source of truth for all CSS (edit this)
- **`tailwind.config.js`** - Theme, colors, plugins (edit this)
- **`static/css/tailwind.css`** - Generated output (do not edit manually)

---

## Getting Started

### Prerequisites

- **Rust** (latest stable) - [Install](https://rustup.rs/)
- **Tailwind CSS CLI** (standalone binary) - [Download](https://github.com/tailwindlabs/tailwindcss/releases)

### Installation

#### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Step 2: Install Tailwind CSS CLI

Choose one method:

**macOS (Homebrew):**

```bash
brew install tailwindcss
```

**Linux (Direct Download):**

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
```

**Manual:** Download from [GitHub Releases](https://github.com/tailwindlabs/tailwindcss/releases)

### Quick Start

```bash
# Terminal 1: Start CSS watch mode (auto-recompile on changes)
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Terminal 2: Run the server
cargo run
```

Visit `http://localhost:3000` in your browser.

---

## Development Guide

### Development Workflow

**Recommended setup (2 terminals):**

```bash
# Terminal 1: CSS watch mode (auto-recompiles on file changes)
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Terminal 2: Run server
cargo run
```

**With auto-reload (optional):**

```bash
# Install cargo-watch once
cargo install cargo-watch

# Use in Terminal 2 instead of cargo run
cargo watch -x run
```

### CSS Development Workflow

**Making style changes:**

1. Edit `src/input.css` or `tailwind.config.js`
2. Watch mode automatically recompiles to `static/css/tailwind.css`
3. Refresh browser to see changes

**Adding Tailwind classes to templates:**

1. Edit any `.html` file in `templates/`
2. Tailwind watch mode detects new classes
3. Automatically includes them in compiled CSS
4. Unused classes are purged automatically

**Production CSS build:**

```bash
tailwindcss -i src/input.css -o static/css/tailwind.css --minify
```

### Common Development Tasks

**Start server:**

```bash
cargo run
# Server starts at http://127.0.0.1:3000
```

**Build for production:**

```bash
cargo build --release
./target/release/ppst-academy
```

**Stop server:**

Press `Ctrl+C` in terminal

---

## Version Control

This project uses [Jujutsu (jj)](https://github.com/martinvonz/jj), a Git-compatible VCS with automatic change tracking.

### Essential Commands

```bash
# View status and changes
jj status                    # Check current status
jj diff                      # View uncommitted changes
jj log                       # View commit history (@ marks current)

# Make commits
jj describe -m "type: message"   # Describe current changes
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

Use conventional commits: `type: description`

**Types:**

- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code refactoring
- `docs:` Documentation
- `style:` CSS/formatting
- `test:` Tests
- `chore:` Maintenance

**Examples:**

```bash
jj describe -m "feat: add contact form validation"
jj describe -m "fix: correct email regex"
jj describe -m "style: update homepage colors"
```

### Typical Workflow

```bash
# 1. Make changes to code/templates/CSS
# 2. Test
cargo test && cargo run

# 3. Commit
jj describe -m "feat: add new section"
jj new

# 4. Push
jj branch set main
jj git push
```

---

## Testing

### Run Tests

```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # With output
```

### Code Quality

```bash
cargo clippy                  # Linter
cargo fmt                     # Auto-format
cargo fmt -- --check          # Check formatting
```

### Manual Testing Checklist

- [ ] Server starts without errors
- [ ] Homepage loads at `http://localhost:3000`
- [ ] Contact form displays and validates correctly
- [ ] Success/error messages appear appropriately
- [ ] Contact data saves to `data/contacts/`
- [ ] CSS styles render correctly across pages

---

## Troubleshooting

### Port Already in Use

```bash
lsof -i :3000              # Find process on port 3000
kill -9 <PID>              # Kill the process
```

### Build Issues

```bash
cargo clean                # Clean artifacts
cargo build                # Rebuild
```

### CSS Not Updating

Ensure Tailwind watch mode is running:

```bash
tailwindcss -i src/input.css -o static/css/tailwind.css --watch
```

Or rebuild manually:

```bash
tailwindcss -i src/input.css -o static/css/tailwind.css --minify
```

### Version Control Issues

**"No changes to describe"** - Make file changes before running `jj describe`

**Push rejected:**

```bash
jj git fetch               # Fetch latest
jj rebase -d main          # Rebase on main
jj git push                # Try again
```

**Reset to remote (⚠️ discards local changes):**

```bash
jj git fetch
jj new main@origin
jj branch set main
```

---

## API Reference

### Endpoints

| Method | Path       | Description                                |
|--------|------------|--------------------------------------------|
| GET    | `/`        | Homepage with academy info and contact form |
| POST   | `/contact` | Submit contact form (returns partial HTML) |

### Data Storage

Contact submissions stored as timestamped JSON files:

**Pattern:** `data/contacts/2024-11-06T12-30-45-123Z.json`

**Example:**

```json
{
  "name": "홍길동",
  "email": "hong@example.com",
  "message": "학원에 대해 궁금한 점이 있습니다.",
  "timestamp": "2024-11-06T12:30:45.123Z"
}
```

---

## Quick Reference

### Daily Commands

```bash
# Development
tailwindcss -i src/input.css -o static/css/tailwind.css --watch
cargo run

# Version Control
jj status
jj describe -m "type: message"
jj new
jj branch set main
jj git push

# Quality
cargo test
cargo clippy
cargo fmt
```

### Project Info

- **Server**: `http://localhost:3000`
- **CSS Source**: `src/input.css` (edit this)
- **CSS Config**: `tailwind.config.js` (edit this)
- **CSS Output**: `static/css/tailwind.css` (generated)
- **Data Storage**: `data/contacts/` (JSON files)
- **Remote**: `git@github.com:hakchin/ppst.git`

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [HTMX Docs](https://htmx.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Jujutsu VCS](https://github.com/martinvonz/jj)

---

## License

MIT

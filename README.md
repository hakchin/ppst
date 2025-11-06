# PPST Academy Website

Educational website for academy/school promotion (학원 홍보 홈페이지)

## Table of Contents

- [Overview](#overview)
- [Technology Stack](#technology-stack)
- [Core Principles](#core-principles)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Development](#development)
- [Testing](#testing)
- [Version Control with Jujutsu](#version-control-with-jujutsu)
- [Common Workflows](#common-workflows)
- [Troubleshooting](#troubleshooting)
- [API Reference](#api-reference)
- [Additional Resources](#additional-resources)

---

## Overview

A modern, lightweight educational website built with Rust and HTMX, following server-side first principles. This is a **learning project** focused on hands-on experience with HTMX-based web development using a minimal, focused implementation approach.

**Key Features:**
- Academy information display with mission, programs, and instructors
- Contact form with HTMX-enhanced validation
- File-based JSON storage (no database)

---

## Technology Stack

### Backend
- **Language**: Rust
- **Web Framework**: Axum 0.7
- **Templating Engine**: Askama 0.12
- **Async Runtime**: Tokio 1.0
- **Serialization**: Serde 1.0
- **Date/Time**: Chrono 0.4
- **Static Files**: Tower-HTTP 0.5

### Frontend
- **Server Communication**: HTMX (served from `static/js/`)
- **Client-Side Interactivity**: Alpine.js (minimal usage, when necessary)
- **Styling**: Tailwind CSS (standalone CLI)

### Build & Development Tools
- **CSS Build**: Tailwind CLI standalone binary (no Node.js)
- **Version Control**: Jujutsu (jj) - Git-compatible VCS
- **Package Manager**: Cargo

---

## Core Principles

### Design Philosophy
- ✅ **Server-Side First** - HTML rendered on server, minimal client-side JavaScript
- ✅ **Zero Node.js** - No npm, webpack, vite, rollup, or bundlers
- ✅ **No Database** - File-based JSON storage for simplicity
- ✅ **Minimal JavaScript** - HTMX for enhancement, Alpine.js only when necessary
- ✅ **Progressive Enhancement** - Core functionality works without JavaScript
- ✅ **Simple Tooling** - Tailwind CLI standalone only
- ✅ **Learning-Oriented** - Clean, educational code structure

### Development Constraints

**JavaScript Usage:**
- Minimize JavaScript wherever possible
- Rely primarily on HTMX for dynamic interactions
- Use Alpine.js only when client-side state management is absolutely necessary

**Data Management:**
- No database systems (SQL or NoSQL)
- Static content managed through code/templates
- User data stored as individual JSON files with timestamp-based filenames
- Contact submissions: `data/contacts/2024-11-06T12-30-45-123Z.json`

**Technology Choices:**
- **HTML**: Core structure and content
- **CSS (Tailwind)**: Styling via standalone CLI (no Node.js)
- **Server-side rendering**: Askama templates
- **HTMX**: Local static file from `static/js/` (no CDN)

---

## Features

- **Homepage**: Academy information with mission, programs, and instructors
- **Contact Form**: HTMX-enhanced form with server-side validation
- **File-Based Storage**: Contact submissions saved as timestamped JSON files
- **Responsive Design**: Mobile-first Tailwind CSS styling
- **Progressive Enhancement**: Works without JavaScript enabled

## Project Structure

```text
ppst/
├── src/
│   ├── main.rs           # Application entry point
│   ├── routes.rs         # Route definitions
│   ├── input.css         # Tailwind input styles
│   ├── handlers/         # Request handlers
│   │   ├── homepage.rs   # Homepage handler
│   │   └── contact.rs    # Contact form handler
│   ├── models/           # Data structures
│   │   ├── academy.rs    # Academy info model
│   │   └── contact.rs    # Contact form model
│   └── storage/          # File storage utilities
│       └── file_store.rs # JSON file operations
├── templates/
│   ├── base.html         # Base layout
│   ├── homepage.html     # Homepage template
│   └── partials/         # Reusable components
│       ├── header.html
│       ├── mission.html
│       ├── programs.html
│       ├── instructors.html
│       ├── contact_form.html
│       ├── contact_success.html
│       └── contact_error.html
├── static/
│   ├── css/
│   │   └── tailwind.css  # Compiled Tailwind CSS
│   └── js/
│       └── htmx.min.js   # HTMX library
├── data/
│   └── contacts/         # Contact submissions (JSON)
├── Cargo.toml            # Rust dependencies
└── tailwind.config.js    # Tailwind configuration
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Tailwind CSS CLI](https://tailwindcss.com/blog/standalone-cli) (standalone binary)

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) - Latest stable toolchain
- [Cargo](https://doc.rust-lang.org/cargo/) - Comes with Rust
- [Tailwind CSS CLI](https://tailwindcss.com/blog/standalone-cli) - Standalone binary

### Installation

#### 1. Install Rust

```bash
# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Install Tailwind CSS CLI

**Option 1: macOS via Homebrew**
```bash
brew install tailwindcss
```

**Option 2: Linux - Download Binary Directly**
```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
```

**Option 3: Manual Download**
- Visit: https://github.com/tailwindlabs/tailwindcss/releases
- Download the binary for your OS/architecture
- Make it executable and add to your PATH

The CLI automatically reads `tailwind.config.js` from the project root.

### Quick Start

```bash
# Terminal 1: Watch and compile Tailwind CSS
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Terminal 2: Run the application
cargo run

# The server will start at http://localhost:3000
```

Open your browser and navigate to `http://localhost:3000`

---

## Development

### Running the Development Server

**Start the server:**
```bash
cargo run
```

**Expected output:**
```
🚀 PPST Academy server listening on http://127.0.0.1:3000
```

**Stop the server:**
- Press `Ctrl+C` in the terminal

### Tailwind CSS Development

**Build CSS once (for production):**
```bash
tailwindcss -i src/input.css -o static/css/tailwind.css --minify
```

**Watch mode (for development):**
```bash
tailwindcss -i src/input.css -o static/css/tailwind.css --watch
```

This watches for changes in `src/input.css` and templates, automatically rebuilding `static/css/tailwind.css`.

### Auto-Reload Development

Install and use `cargo-watch` for automatic server restart on code changes:

```bash
# Install cargo-watch (one-time)
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Production Build

```bash
# Build optimized release binary
cargo build --release

# Run the production binary
./target/release/ppst-academy
```

---

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Code Quality Checks

**Run Clippy (linter):**
```bash
cargo clippy
```

**Check code formatting:**
```bash
cargo fmt -- --check
```

**Auto-format code:**
```bash
cargo fmt
```

### Manual Testing Checklist

After making changes, verify:

- [ ] Server starts without errors
- [ ] Homepage loads correctly at `http://localhost:3000`
- [ ] Contact form displays properly
- [ ] Form validation works (try invalid email, empty fields)
- [ ] Success message appears after valid submission
- [ ] Contact data is saved to `data/contacts/`
- [ ] Error messages display appropriately

---

## Version Control with Jujutsu

This project uses [Jujutsu (jj)](https://github.com/martinvonz/jj), a Git-compatible version control system. Unlike Git, changes are automatically tracked in real-time.

### Basic Commands

**Check status:**
```bash
jj status
```

**View commit history:**
```bash
jj log
```
Your current working copy is marked with `@`.

**View changes:**
```bash
jj diff
```

### Making Commits

**1. Describe your changes:**
```bash
jj describe -m "feat: add new feature description"
```

**2. Create a new working change:**
```bash
jj new
```

### Commit Message Conventions

Follow the format: `type: description`

**Common types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `chore:` - Maintenance tasks
- `style:` - Formatting, styling changes

**Examples:**
```bash
jj describe -m "feat: add contact form validation"
jj describe -m "fix: correct email regex pattern"
jj describe -m "docs: update README with setup instructions"
```

### Pushing to GitHub

**1. Ensure your changes are committed:**
```bash
jj log  # Verify your changes appear as a commit
```

**2. Update the branch pointer:**
```bash
# For main branch
jj branch set main

# For feature branch
jj branch set feature/my-feature
```

**3. Push to GitHub:**
```bash
# Push current branch
jj git push

# Push specific branch
jj git push --branch feature/my-feature
```

### Undoing Changes

**Restore specific files:**
```bash
jj restore <file-path>
```

**Restore all changes:**
```bash
jj restore --from @-
```

### Updating from Remote

```bash
# Fetch latest changes from GitHub
jj git fetch

# Create a new change based on updated main
jj new main
```

---

## Common Workflows

### Workflow 1: Quick Fix

```bash
# 1. Make your changes
# (edit files)

# 2. Test the fix
cargo test
cargo run

# 3. Commit
jj describe -m "fix: correct form validation error message"
jj new

# 4. Push
jj branch set main
jj git push
```

### Workflow 2: Adding a New Feature

```bash
# 1. Create feature branch
jj branch create feature/instructor-bios
jj branch set feature/instructor-bios

# 2. Implement feature
# (make changes)

# 3. Test thoroughly
cargo test
cargo clippy
cargo run

# 4. Commit
jj describe -m "feat: add instructor bio data model"
jj new

# 5. Push feature branch
jj git push --branch feature/instructor-bios
```

### Workflow 3: Daily Development

```bash
# 1. Make changes
# (edit files)

# 2. Check what changed
jj status
jj diff

# 3. Test changes
cargo test
cargo clippy

# 4. Run server to verify
cargo run
# Test in browser

# 5. Commit changes
jj describe -m "feat: implement instructor profiles section"
jj new

# 6. Update branch and push
jj branch set main
jj git push
```

---

## Troubleshooting

### Server Won't Start

**Error:** `Address already in use`

**Solution:**
```bash
# Find process using port 3000
lsof -i :3000

# Kill the process
kill -9 <PID>
```

### Compilation Errors

```bash
# Clean build artifacts
cargo clean

# Rebuild from scratch
cargo build
```

### Tailwind CSS Not Updating

```bash
# Make sure watch mode is running
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Or rebuild manually
tailwindcss -i src/input.css -o static/css/tailwind.css --minify
```

### Jujutsu: "No changes to describe"

This means your working change is empty. Make file changes first, then describe.

### Push Rejected

If `jj git push` fails due to conflicts:

```bash
# Fetch latest changes
jj git fetch

# Rebase your changes on top of remote
jj rebase -d main

# Try pushing again
jj git push
```

### View Git Commits

```bash
# See the underlying Git commits
jj log --template 'commit_id ++ " " ++ description'
```

### Reset to Remote State

**⚠️ Warning: This discards local changes!**

```bash
jj git fetch
jj new main@origin
jj branch set main
```

---

## API Reference

### Endpoints

| Method | Path       | Description                                  |
|--------|------------|----------------------------------------------|
| GET    | `/`        | Homepage with academy info and contact form  |
| POST   | `/contact` | Submit contact form (returns partial HTML)   |

### File-Based Storage

Contact form submissions are stored as individual JSON files with ISO 8601 timestamp-based filenames:

**File path pattern:**
```
data/contacts/2024-11-06T12-30-45-123Z.json
```

**Example file content:**
```json
{
  "name": "홍길동",
  "email": "hong@example.com",
  "message": "학원에 대해 궁금한 점이 있습니다.",
  "timestamp": "2024-11-06T12:30:45.123Z"
}
```

---

## Additional Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Axum Web Framework](https://docs.rs/axum/latest/axum/)
- [Askama Templating](https://docs.rs/askama/)
- [HTMX Documentation](https://htmx.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Jujutsu VCS](https://github.com/martinvonz/jj)

### Quick Reference

**Daily commands:**
```bash
# Start server
cargo run

# Check status
jj status

# View changes
jj diff

# Commit changes
jj describe -m "type: message"
jj new

# Push to GitHub
jj branch set main
jj git push
```

**Environment information:**
- **Server URL**: `http://localhost:3000`
- **Port**: `3000`
- **Contact Data**: `data/contacts/`
- **Static Files**: `static/`
- **Templates**: `templates/`
- **Main Branch**: `main`
- **Git Remote**: `git@github.com:hakchin/ppst.git`

---

## Development Guidelines

When working on this project:

1. **Prioritize HTML/CSS** solutions over JavaScript
2. **Use HTMX attributes** for dynamic behavior
3. **Keep Alpine.js usage minimal** and justified
4. **Follow Rust best practices** for Axum handlers
5. **Maintain clean code structure** for educational purposes
6. **Avoid database dependencies** - use file-based storage
7. **Use Askama templates** for server-side rendering
8. **Never introduce Node.js, npm, or bundlers** (webpack/vite/rollup)
9. **Use Tailwind CLI standalone** binary only (via brew or GitHub releases)
10. **Serve HTMX locally** from `static/js/` (no CDN dependencies)
11. **Store data as JSON files** with timestamp-based filenames in `data/contacts/`

---

## Project Goals

This is a learning project focused on:
- Modern Rust web development with Axum
- Server-side rendering with minimal JavaScript
- HTMX for progressive enhancement
- Simple, maintainable architecture without complex build tools
- Educational code quality and clear patterns
- File-based data storage without database complexity

---

## License

MIT

---

**Last Updated**: 2024-11-06  
**Project**: PPST Academy  
**Version Control**: Jujutsu (Git-backed)

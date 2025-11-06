# PPST Academy Website

Educational website for academy/school promotion (학원 홍보 홈페이지)

## Overview

A modern, lightweight website built with Rust and HTMX, following server-side first principles. Features academy information display and a contact form with file-based storage.

## Technology Stack

- **Backend**: Rust + Axum
- **Templating**: Askama
- **Frontend**: HTMX (minimal JavaScript)
- **Styling**: Tailwind CSS
- **Storage**: File-based JSON (no database)
- **Build**: Tailwind CLI standalone

## Core Principles

- **Server-Side First**: HTML rendered on server
- **Zero Node.js**: No npm, no build complexity
- **No Database**: File-based JSON storage
- **Minimal JavaScript**: HTMX for enhancement only
- **Progressive Enhancement**: Works without JavaScript

## Features

- **Homepage**: Academy information with mission, programs, and instructors
- **Contact Form**: HTMX-enhanced form with validation
- **File-Based Storage**: Contact submissions saved as timestamped JSON files

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

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Tailwind CSS CLI (macOS/Linux)
# Download from https://github.com/tailwindlabs/tailwindcss/releases
# Or via package manager:
# macOS: brew install tailwindcss
# Linux: npm install -g tailwindcss-cli (if you have npm, otherwise use binary)
```

### Build & Run

```bash
# Terminal 1: Watch and compile Tailwind CSS
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Terminal 2: Run the application
cargo run

# The server will start at http://localhost:3000
```

### Development

```bash
# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Build for production
cargo build --release
./target/release/ppst-academy
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Homepage with academy info and contact form |
| POST | `/contact` | Submit contact form (returns partial HTML) |

## File-Based Storage

Contact form submissions are stored as timestamped JSON files:

```bash
data/contacts/2024-11-06T12-30-45-123Z.json
```

Example file content:
```json
{
  "name": "홍길동",
  "email": "hong@example.com",
  "message": "학원에 대해 궁금한 점이 있습니다.",
  "timestamp": "2024-11-06T12:30:45.123Z"
}
```

## Key Dependencies

- **axum** 0.7 - Web framework
- **tokio** 1.0 - Async runtime
- **askama** 0.12 - HTML templating
- **serde** 1.0 - Serialization
- **chrono** 0.4 - Date/time handling
- **tower-http** 0.5 - Static file serving

## Project Goals

This is a learning project focused on:
- Modern Rust web development with Axum
- Server-side rendering with minimal JavaScript
- HTMX for progressive enhancement
- Simple, maintainable architecture
- Educational code quality

## License

MIT

## Links

- [Axum Documentation](https://docs.rs/axum/)
- [Askama Documentation](https://docs.rs/askama/)
- [HTMX Documentation](https://htmx.org/)
- [Tailwind CSS](https://tailwindcss.com/)

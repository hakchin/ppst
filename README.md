# PPST Academy Website

Educational website for academy/school promotion (학원 홍보 홈페이지)

## Technology Stack

- **Backend**: Rust + Axum
- **Templating**: Askama
- **Frontend Enhancement**: HTMX
- **Styling**: Tailwind CSS
- **Build Tool**: Tailwind CLI standalone

## Core Principles

- Zero Node.js/npm dependencies
- No databases (file-based JSON storage)
- Minimal JavaScript (HTMX for enhancement only)
- Server-side rendering first

## Project Structure

```text
src/                       # Rust source code
  ├── handlers/            # Axum request handlers
  ├── models/              # Data models and validation
  └── storage/             # File-based storage logic
templates/                 # Askama HTML templates
  ├── base.html            # Base layout
  ├── *.html               # Page templates
  └── partials/            # Reusable components
static/                    # Static assets
  ├── css/                 # Compiled Tailwind CSS
  └── js/                  # HTMX library
data/                      # File-based data storage
  └── contacts/            # Contact form submissions (JSON)
```

## Getting Started

### Prerequisites

- Rust (stable toolchain)
- Tailwind CSS standalone CLI

### Build & Run

```bash
# Build CSS
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Build and run (in separate terminal)
cargo run
```

## File-Based Storage

Contact submissions are stored as timestamped JSON files:

```
data/contacts/2024-11-06T12-30-45-123Z.json
```

## License

MIT

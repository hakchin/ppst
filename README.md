# PPST Academy Website

Educational website for academy/school promotion (학원 홍보 홈페이지)

## Project Overview

Personal learning project focused on HTMX-based web development with minimal, educational approach.

**Core Philosophy**:
- Zero Node.js/npm dependencies
- No JavaScript bundlers or build complexity
- No databases (file-based storage only)
- Minimal JavaScript (HTMX + Alpine.js when absolutely necessary)
- Simple tooling (Tailwind CLI standalone only)
- Educational code quality (clear, instructive patterns)

## Technology Stack

- **Backend**: Rust + Axum
- **Templating**: Askama
- **Frontend Enhancement**: HTMX
- **Minimal Interactivity**: Alpine.js (restricted use)
- **Styling**: Tailwind CSS
- **Build Tool**: Tailwind CLI standalone
- **Version Control**: Jujutsu (jj) with Git

## Development with GitHub Copilot

This project uses GitHub Copilot for specification-driven development with custom instruction files.

### Quick Start

Use `@workspace` in GitHub Copilot Chat:

```text
# Create a new feature specification
@workspace Create specification for: homepage with contact form

# Generate implementation plan
@workspace Create plan for: spec-001-homepage-contact

# Break down into tasks
@workspace Create tasks for: plan-001-homepage-contact

# Update project constitution
@workspace Update the project constitution: add security requirements
```

### Workflow

1. **Specify**: Define feature from user perspective
2. **Plan**: Create implementation roadmap
3. **Task**: Break down into actionable items
4. **Implement**: Follow tasks, check off completion
5. **Review**: Verify constitution compliance

See [`.github/copilot/README.md`](.github/copilot/README.md) for detailed instructions.

## Project Structure

```text
.github/copilot/           # GitHub Copilot instruction files
.specify/
  ├── memory/
  │   └── constitution.md  # Project governance and principles
  ├── specs/               # Feature specifications
  ├── plans/               # Implementation plans
  ├── tasks/               # Task breakdowns
  └── templates/           # Document templates
src/                       # Rust source code
  ├── handlers/            # Axum request handlers
  ├── models/              # Data models and validation
  └── storage/             # File-based storage logic
templates/                 # Askama HTML templates
  ├── pages/               # Full page templates
  ├── partials/            # Reusable components
  └── layouts/             # Base layouts
static/                    # Static assets
  ├── css/                 # Compiled Tailwind CSS
  └── js/                  # HTMX (static file)
data/                      # File-based data storage
  └── contacts/            # Contact form submissions (JSON)
```

## Constitution Principles

This project follows strict principles (see [`.specify/memory/constitution.md`](.specify/memory/constitution.md)):

1. **Server-Side First**: HTML-over-the-wire is default, justify any client-side logic
2. **Static Content Architecture**: No databases, use timestamped JSON files
3. **HTML/CSS Over JavaScript**: Minimize JS, HTMX for enhancement only
4. **Educational Code Quality**: Clear, instructive code over optimization
5. **Progressive Enhancement**: Works without JavaScript

## Getting Started

### Prerequisites

- Rust (stable toolchain)
- Tailwind CSS standalone CLI
- Jujutsu (jj) - optional but recommended

### Installation

**macOS**:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Tailwind CSS standalone
brew install tailwindcss

# Install Jujutsu (optional)
brew install jj
```

### Build & Run

```bash
# Clone repository
git clone https://github.com/hakchin/ppst.git
cd ppst

# Build CSS
tailwindcss -i src/input.css -o static/css/tailwind.css --watch

# Build and run (in separate terminal)
cargo run

# Development mode with auto-reload
cargo watch -x run
```

### Directory Setup

```bash
# Create data directories
mkdir -p data/contacts

# Set appropriate permissions
chmod 700 data/contacts
```

## Development Guidelines

### For AI Assistants (GitHub Copilot)

When working on this project:

1. **Always check constitution compliance** before suggesting implementations
2. **Prioritize server-side solutions** over client-side JavaScript
3. **Use HTMX for dynamic behavior**, not Alpine.js (unless absolutely necessary)
4. **Follow Rust best practices** for Axum handlers and Askama templates
5. **Maintain educational code quality** with clear patterns and comments
6. **Avoid database dependencies** - use file-based storage per constitution
7. **Verify progressive enhancement** - feature must work without JavaScript

### Constitution Compliance

Every implementation must pass:

- ✅ Server-side rendering with Askama templates
- ✅ No database dependencies (use file storage)
- ✅ JavaScript minimized (HTMX only for enhancement)
- ✅ Educational code patterns (clear and instructive)
- ✅ Works without JavaScript enabled

### File-Based Storage Pattern

Contact form submissions example:

```rust
// Timestamp-based filename: YYYY-MM-DDTHH-MM-SS-mmmZ.json
let filename = format!("{}.json", Utc::now().format("%Y-%m-%dT%H-%M-%S-%3fZ"));
let path = PathBuf::from("data/contacts").join(filename);

// Serialize to JSON
let json = serde_json::to_string_pretty(&submission)?;
fs::write(path, json)?;
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Deployment

TBD - Deployment strategy will be defined in future specifications.

## Contributing

This is a personal learning project. For contributions or suggestions, please open an issue first to discuss proposed changes.

## License

MIT License - See LICENSE file for details

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Askama Documentation](https://docs.rs/askama/)
- [HTMX Documentation](https://htmx.org/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [Jujutsu Documentation](https://martinvonz.github.io/jj/)

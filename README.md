# PPST Academy

A modern educational website built with Rust full-stack technologies.

## Key Features

- **Server-Side Rendering (SSR)** with client-side hydration for optimal performance and SEO
- **Fine-grained Reactivity** using Leptos signals for efficient UI updates
- **Type-safe Full-stack** with shared Rust code between server and client
- **Server Functions** for seamless client-server communication without manual API routes
- **File-based Storage** for simple, database-free data persistence
- **Responsive Design** with utility-first CSS approach
- **Zero JavaScript Dependencies** - all interactivity powered by Rust/WASM

## Design Philosophy

### Server-First, Client-Enhanced

The application renders complete HTML on the server for fast initial page loads and SEO optimization. Client-side hydration then enables rich interactivity without full page reloads.

### Type Safety Everywhere

Rust's type system ensures correctness at compile time - from database models to UI components. No runtime type errors, no `undefined is not a function`.

### Minimal Dependencies

We prefer fewer, well-chosen dependencies over a bloated node_modules. The entire frontend compiles to a single WASM binary with no external JavaScript libraries.

### Progressive Enhancement

Core functionality works without JavaScript. Enhanced features gracefully layer on top when WASM loads.

### Simplicity Over Complexity

File-based JSON storage instead of database complexity. Utility CSS instead of elaborate class hierarchies. Convention over configuration.

## Technology Stack

### Backend

| Technology | Purpose |
|------------|---------|
| **Rust** | Systems programming language |
| **Axum 0.8** | Async web framework |
| **Tokio** | Async runtime |
| **Leptos 0.8** | Full-stack reactive framework |
| **Tower-HTTP** | HTTP middleware (compression, static files) |

### Frontend

| Technology | Purpose |
|------------|---------|
| **Leptos** | Reactive UI components |
| **Tailwind CSS v4** | Utility-first styling |
| **WebAssembly** | Client-side Rust execution |

### Tooling

| Tool | Purpose |
|------|---------|
| **cargo-leptos** | Build orchestration (SSR + WASM + Tailwind) |
| **Tailwind CLI** | CSS compilation (standalone, no Node.js) |

### Data & Serialization

| Technology | Purpose |
|------------|---------|
| **Serde** | Serialization/deserialization |
| **serde_json** | JSON parsing |
| **File System** | JSON-based data storage |

### Quality & Observability

| Tool | Purpose |
|------|---------|
| **tracing** | Structured logging |
| **thiserror** | Error handling |
| **clippy** | Linting |
| **rustfmt** | Code formatting |

## Quick Start

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos

# Development server with hot reload
cargo leptos watch

# Production build
cargo leptos build --release

# Run production server
./target/release/ppst-academy
```

## Project Structure

```
ppst/
├── src/
│   ├── lib.rs              # Library crate (shared code)
│   ├── main.rs             # Server binary
│   ├── app.rs              # Root App component
│   ├── components/         # Reusable UI components
│   ├── pages/              # Route page components
│   ├── server/             # Server-only code
│   └── models/             # Shared data types
├── public/                 # Static assets
├── style/                  # Generated CSS
├── data/                   # JSON storage
├── tailwind.config.js      # Tailwind configuration
├── input.css               # Tailwind input
├── Cargo.toml              # Dependencies
└── CLAUDE.md               # AI assistant guidelines
```

## JavaScript Exclusion Strategy

The project strictly adheres to a "No JavaScript" policy for source code, relying on Rust/WASM for all client-side logic.

1.  **Tailwind CSS**: Used only for build-time CSS generation. No runtime JS dependency.
2.  **Browser Interop**: If browser APIs are needed (e.g., LocalStorage), use Rust wrappers like `gloo` or `leptos-use` instead of writing raw JS.
3.  **Dependencies**: Avoid adding npm packages. Prefer Rust crates that compile to WASM.

## License

MIT

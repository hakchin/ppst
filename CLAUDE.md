# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **full-stack Rust educational website** for PPST Academy using Leptos and Tailwind CSS. The project emphasizes type safety, server-side rendering with hydration, and minimal JavaScript dependencies.

**Philosophy**: Full-stack Rust with SSR + hydration, file-based JSON storage, utility-first CSS with Tailwind, and WASM-powered client-side interactivity.

## Essential Commands

### Development

```bash
# Run development server with hot reload (http://0.0.0.0:3000)
cargo leptos watch

# Build for development
cargo leptos build

# Production build
cargo leptos build --release

# Run production server
./target/release/ppst-academy
```

### Testing & Quality

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check SSR feature compilation
cargo check --features ssr

# Check hydrate feature compilation
cargo check --features hydrate

# Lint code
cargo clippy --features ssr
cargo clippy --features hydrate

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Tailwind CSS

```bash
# Generate CSS (usually handled by cargo-leptos)
tailwindcss -i input.css -o style/output.css

# Watch for changes
tailwindcss -i input.css -o style/output.css --watch
```

### Version Control (Git)

```bash
# View status and changes
git status
git diff
git log

# Commit changes
git add .
git commit -m "type: message"

# Sync with remote
git fetch
git pull
git push
```

**Commit format**: Use conventional commits (`feat:`, `fix:`, `style:`, `refactor:`, `docs:`, `test:`, `chore:`)

## Architecture

### Request Flow

```text
Browser Request
    ↓
Axum Router (src/main.rs)
    ↓
Leptos SSR (renders App component)
    ↓
HTML Response with WASM bundle
    ↓
Client Hydration (src/lib.rs)
    ↓
Interactive SPA
```

### Module Structure

```text
src/
├── lib.rs              # Library crate, hydrate entry point
├── main.rs             # Server binary, Axum setup
├── app.rs              # Root App component with Router
├── constants.rs        # Application-wide constants (contact info)
├── components/         # Reusable UI components (categorized)
│   ├── layout/         # Page structure (header, footer)
│   ├── ui/             # Visual primitives (icons)
│   └── maps/           # Location components (directions)
├── pages/              # Route page components
│   ├── home/           # Landing page (modular sections)
│   │   ├── hero.rs, mission.rs, achievements.rs
│   │   ├── teaching.rs, programs.rs, admissions.rs
│   │   ├── policies.rs, contact.rs
│   │   └── mod.rs      # Composes all sections into HomePage
│   ├── about.rs        # About page
│   └── not_found.rs    # 404 page
├── models/             # Shared data types
│   └── contact.rs      # Contact inquiry model
└── server/             # Server-only code (SSR feature)
    └── file_store.rs   # JSON file storage
```

### Feature Flags

The project uses Cargo features for conditional compilation:

| Feature   | Purpose                               | Enabled By      |
| --------- | ------------------------------------- | --------------- |
| `ssr`     | Server-side rendering, Axum, file I/O | Server binary   |
| `hydrate` | Client-side hydration, WASM           | Frontend bundle |
| `csr`     | Client-side only rendering            | Not used        |

### Rendering Modes

1. **Server-Side Rendering (SSR)**
   - Initial HTML rendered on server
   - SEO-friendly, fast first paint
   - Configured in `src/main.rs`

1. **Hydration**
   - WASM bundle attaches to server-rendered HTML
   - Enables client-side interactivity
   - Entry point in `src/lib.rs`

## Leptos Patterns

### Component Definition

```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent(
    #[prop(into)] title: String,           // Required prop
    #[prop(default = "default")] variant: &'static str,  // Optional with default
    #[prop(optional)] children: Option<Children>,        // Optional children
) -> impl IntoView {
    view! {
        <div class="my-component">
            <h1>{title}</h1>
            {children.map(|c| c())}
        </div>
    }
}
```

### Reactive Signals

```rust
// Create a signal (reactive state)
let (count, set_count) = signal(0);

// Read signal value
let value = count.get();

// Update signal
set_count.set(value + 1);
set_count.update(|v| *v += 1);

// Derive computed values
let doubled = move || count.get() * 2;
```

### Router Links

Use `<A>` component from `leptos_router` with `attr:class` for styling:

```rust
use leptos_router::components::A;

view! {
    <A href="/about" attr:class="text-blue-600 hover:underline">
        "About Us"
    </A>
}
```

### Server Functions

```rust
#[server]
pub async fn my_server_fn(arg: String) -> Result<Data, ServerFnError> {
    // This code runs ONLY on the server
    // Access databases, file system, etc.
    Ok(Data { ... })
}

// Call from component
let action = Action::new(|input: &String| {
    my_server_fn(input.clone())
});
action.dispatch("value".to_string());
```

## Tailwind CSS v4

### Configuration

Tailwind v4 uses CSS-native configuration in `input.css` (no JavaScript config file):

```css
@import "tailwindcss";

/* Content sources for class scanning */
@source "./src/**/*.rs";
@source "./public/**/*.html";

@theme {
  --color-brand-500: #3b82f6;
  --color-brand-600: #2563eb;
  --font-sans: "Pretendard", system-ui, sans-serif;
}
```

### Custom Components (DRY Principle)

**IMPORTANT**: Follow the DRY (Don't Repeat Yourself) principle. Define reusable component classes in `input.css` instead of repeating inline utility combinations.

Available component classes:

| Class                    | Purpose                                                 |
| ------------------------ | ------------------------------------------------------- |
| `.section-title`         | Section heading (`text-3xl md:text-4xl font-bold mb-4`) |
| `.section-subtitle`      | Section quote/subtitle with left border                 |
| `.card-highlight`        | Brand-colored highlight card                            |
| `.btn-primary`           | Primary action button                                   |
| `.btn-secondary-inverse` | White button for dark backgrounds                       |
| `.form-input`            | Form text input                                         |
| `.form-textarea`         | Form textarea                                           |
| `.container-section`     | Page section container                                  |

```css
@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-brand-600 text-white rounded-lg hover:bg-brand-700;
  }
}
```

**When to create a new class**: If you find yourself writing the same utility combination 3+ times, extract it to `input.css`.

### Using in Rust

```rust
view! {
    <button class="btn-primary">"Click me"</button>
    <div class="flex items-center gap-4 p-6 bg-gray-50 rounded-xl">
        // Content
    </div>
}
```

## Data Storage

### File-based JSON Storage

Contact inquiries stored in `data/contacts/`:

- Pattern: `YYYY-MM-DDTHH-MM-SS-mmmZ.json`
- Auto-creates directories
- Pretty-printed JSON

### Contact Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInquiry {
    pub name: String,
    pub email: String,
    pub message: String,
    pub submitted_at: OffsetDateTime,
}
```

## Key Technologies

### Backend

| Technology   | Version      | Purpose             |
| ------------ | ------------ | ------------------- |
| Rust         | 2024 Edition | Systems programming |
| Leptos       | 0.8          | Full-stack framework|
| Axum         | 0.8          | HTTP server         |
| Tokio        | 1.x          | Async runtime       |
| Tower-HTTP   | 0.6          | HTTP middleware     |

### Frontend

| Technology   | Purpose                    |
| ------------ | -------------------------- |
| Leptos       | Reactive UI components     |
| Tailwind CSS | v4 utility-first styling   |
| WebAssembly  | Client-side Rust execution |

### Tooling

| Tool         | Purpose             |
| ------------ | ------------------- |
| cargo-leptos | Build orchestration |
| tailwindcss  | CSS compilation     |
| wasm-bindgen | WASM JS bindings    |

## Port Configuration

Default: `0.0.0.0:3000` (accessible from network)

Configure in `Cargo.toml`:

```toml
[package.metadata.leptos]
site-addr = "0.0.0.0:3000"
```

Or via environment:

```bash
LEPTOS_SITE_ADDR=127.0.0.1:3000 cargo leptos watch  # localhost only
```

## Development Patterns

### Adding a New Page

1. Create page component in `src/pages/new_page.rs`:

```rust
use leptos::prelude::*;

#[component]
pub fn NewPage() -> impl IntoView {
    view! {
        <div class="container-section py-16">
            <h1 class="text-4xl font-bold">"New Page"</h1>
        </div>
    }
}
```

1. Export in `src/pages/mod.rs`:

```rust
mod new_page;
pub use new_page::NewPage;
```

1. Add route in `src/app.rs`:

```rust
<Route path=path!("/new-page") view=NewPage/>
```

### Adding a Component

1. Create in `src/components/my_component.rs`
1. Export in `src/components/mod.rs`
1. Use in pages: `<MyComponent prop="value"/>`

### Adding Server Functions

1. Define in appropriate module with `#[server]` macro
1. Ensure function is `async` and returns `Result<T, ServerFnError>`
1. Call from components using `Action::new()`

## File References

- **Entry points**: `src/main.rs` (server), `src/lib.rs` (client)
- **Root component**: `src/app.rs`
- **Tailwind input**: `input.css`
- **Tailwind output**: `style/output.css`
- **Leptos config**: `Cargo.toml` `[package.metadata.leptos]`

## Use Context7 MCP tools

Always use context7 when I need code generation, setup or configuration steps, or library/API documentation. This means you should automatically use the Context7 MCP tools to resolve library id and get library docs without me having to explicitly ask.

## Important Notes

- **Rust Edition 2024**: Uses latest stable Rust features
- **Zero JavaScript**: No JS files in the project; Tailwind CLI is standalone binary
- **Type Safety**: Components and server functions are compile-time checked
- **SSR + Hydration**: Full page rendered on server, then hydrated
- **`attr:class`**: Use this syntax for class attributes on Leptos router components (`<A>`)
- **Constants**: Use `src/constants.rs` for shared values (contact info, etc.) to maintain DRY

# Quickstart Guide: Academy Homepage

**Feature**: 001-homepage
**Date**: 2025-10-30
**For**: Developers implementing this feature

## Overview

This guide provides step-by-step instructions for implementing the Academy Homepage feature. Follow these steps in order to build a working homepage with academy information and contact form functionality.

---

## Prerequisites

Before starting, ensure you have:

- **Rust**: Stable channel (1.70+)
  ```bash
  rustc --version  # Should show 1.70 or higher
  ```

- **Cargo**: Comes with Rust
  ```bash
  cargo --version
  ```

- **Node.js/npm**: For Tailwind CSS (16+ recommended)
  ```bash
  node --version
  npm --version
  ```

- **Git/jj**: For version control
  ```bash
  jj --version
  ```

---

## Step 1: Project Initialization

### 1.1 Create Rust Project Structure

If not already done, initialize the project:

```bash
# Create src directory structure
mkdir -p src/{handlers,models,storage,templates/partials}
mkdir -p static/{css,js,images/instructors}
mkdir -p data/contacts
mkdir -p tests/{unit,integration}

# Create placeholder files
touch src/main.rs
touch src/routes.rs
touch src/handlers/mod.rs
touch src/handlers/homepage.rs
touch src/handlers/contact.rs
touch src/models/mod.rs
touch src/models/contact.rs
touch src/storage/mod.rs
touch src/storage/file_store.rs
touch data/contacts/.gitkeep
```

### 1.2 Configure Cargo.toml

Create or update `Cargo.toml`:

```toml
[package]
name = "ppst-academy"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "trace"] }

# Templating
askama = "0.12"
askama_axum = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Validation
regex = "1.10"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
# Testing
reqwest = { version = "0.11", features = ["json"] }
```

---

## Step 2: Tailwind CSS Setup

### 2.1 Initialize Tailwind

```bash
# Install Tailwind CLI (if not already installed)
npm install -D tailwindcss

# Initialize Tailwind config
npx tailwindcss init
```

### 2.2 Configure Tailwind

Edit `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        'academy-blue': '#1e40af',
        'academy-gold': '#f59e0b',
      }
    },
  },
  plugins: [],
}
```

### 2.3 Create Tailwind Input File

Create `input.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom styles if needed */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.sr-only:focus {
  position: static;
  width: auto;
  height: auto;
  padding: inherit;
  margin: inherit;
  overflow: visible;
  clip: auto;
  white-space: normal;
}
```

### 2.4 Build Tailwind CSS

```bash
# Development (watch mode)
npx tailwindcss -i ./input.css -o ./static/css/tailwind.css --watch

# Production (minified)
npx tailwindcss -i ./input.css -o ./static/css/tailwind.css --minify
```

**Tip**: Add to `package.json` scripts:
```json
{
  "scripts": {
    "css:watch": "tailwindcss -i ./input.css -o ./static/css/tailwind.css --watch",
    "css:build": "tailwindcss -i ./input.css -o ./static/css/tailwind.css --minify"
  }
}
```

---

## Step 3: Download HTMX

```bash
# Download HTMX (version 1.9+)
curl -o static/js/htmx.min.js https://unpkg.com/htmx.org@1.9.10/dist/htmx.min.js

# Or use CDN in templates (development)
# <script src="https://unpkg.com/htmx.org@1.9.10"></script>
```

---

## Step 4: Implement Data Models

### 4.1 Contact Model

Edit `src/models/contact.rs`:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInquiry {
    pub id: String,
    pub submitted_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactFormInput {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

impl ContactFormInput {
    pub fn into_inquiry(
        self,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<ContactInquiry, Vec<String>> {
        // Validate
        let errors = self.validate();
        if !errors.is_empty() {
            return Err(errors);
        }

        // Generate ID and timestamp
        let now = Utc::now();
        let id = now.format("%Y-%m-%dT%H-%M-%S-%3fZ").to_string();

        Ok(ContactInquiry {
            id,
            submitted_at: now,
            name: self.name.trim().to_string(),
            email: self.email.trim().to_lowercase(),
            subject: self.subject.filter(|s| !s.trim().is_empty()),
            message: self.message.trim().to_string(),
            user_agent,
            ip_address,
        })
    }

    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push("Name is required".to_string());
        }

        if self.email.trim().is_empty() {
            errors.push("Email is required".to_string());
        } else if !is_valid_email(&self.email) {
            errors.push("Email format is invalid".to_string());
        }

        if self.message.trim().is_empty() {
            errors.push("Message is required".to_string());
        }

        errors
    }
}

fn is_valid_email(email: &str) -> bool {
    use regex::Regex;
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}
```

### 4.2 Models Module

Edit `src/models/mod.rs`:

```rust
pub mod contact;
pub use contact::{ContactInquiry, ContactFormInput};
```

---

## Step 5: Implement File Storage

Edit `src/storage/file_store.rs`:

```rust
use crate::models::ContactInquiry;
use std::fs;
use std::path::Path;

pub async fn save_contact(inquiry: &ContactInquiry) -> Result<(), std::io::Error> {
    // Ensure directory exists
    let dir = Path::new("data/contacts");
    fs::create_dir_all(dir)?;

    // Write to file
    let filename = format!("data/contacts/{}.json", inquiry.id);
    let json = serde_json::to_string_pretty(inquiry)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    fs::write(filename, json)?;

    Ok(())
}
```

Edit `src/storage/mod.rs`:

```rust
pub mod file_store;
pub use file_store::save_contact;
```

---

## Step 6: Create Templates

### 6.1 Base Template

Create `src/templates/base.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}PPST Academy{% endblock %}</title>
    <link rel="stylesheet" href="/static/css/tailwind.css">
    {% block extra_head %}{% endblock %}
</head>
<body class="bg-gray-50">
    <!-- Skip link for accessibility -->
    <a href="#main-content" class="sr-only focus:not-sr-only">
        Skip to main content
    </a>

    {% block content %}{% endblock %}

    <!-- HTMX for progressive enhancement (optional) -->
    <script src="/static/js/htmx.min.js"></script>
    {% block extra_scripts %}{% endblock %}
</body>
</html>
```

### 6.2 Homepage Template

Create `src/templates/homepage.html`:

```html
{% extends "base.html" %}

{% block title %}{{ academy_name }} - Home{% endblock %}

{% block content %}
<!-- Header -->
<header class="bg-white shadow">
    <div class="max-w-7xl mx-auto px-4 py-6">
        <h1 class="text-3xl font-bold text-gray-900">{{ academy_name }}</h1>
        <p class="text-gray-600">{{ tagline }}</p>
    </div>
</header>

<!-- Main content -->
<main id="main-content" class="max-w-7xl mx-auto px-4 py-8">
    <!-- Mission section -->
    <section id="mission" class="mb-12">
        <h2 class="text-2xl font-bold mb-4">Our Mission</h2>
        <p class="text-gray-700 leading-relaxed">{{ mission }}</p>
    </section>

    <!-- Programs section -->
    <section id="programs" class="mb-12">
        <h2 class="text-2xl font-bold mb-4">Programs</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {% for program in programs %}
            <div class="bg-white p-6 rounded-lg shadow">
                <h3 class="font-bold text-lg mb-2">{{ program.name }}</h3>
                <p class="text-gray-600">{{ program.description }}</p>
            </div>
            {% endfor %}
        </div>
    </section>

    <!-- Instructors section -->
    <section id="instructors" class="mb-12">
        <h2 class="text-2xl font-bold mb-4">Our Instructors</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {% for instructor in instructors %}
            <div class="bg-white p-6 rounded-lg shadow">
                <div class="flex items-center mb-4">
                    <img src="{{ instructor.photo_path }}" alt="{{ instructor.name }}" class="w-16 h-16 rounded-full mr-4">
                    <div>
                        <h3 class="font-bold">{{ instructor.name }}</h3>
                        <p class="text-sm text-gray-600">{{ instructor.credentials }}</p>
                    </div>
                </div>
                <p class="text-gray-700">{{ instructor.bio }}</p>
            </div>
            {% endfor %}
        </div>
    </section>

    <!-- Contact form section -->
    <section id="contact" class="mb-12">
        <h2 class="text-2xl font-bold mb-4">Contact Us</h2>

        <!-- Success/error messages -->
        <div id="contact-result" class="mb-4">
            {% if success %}
            <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
                <p class="font-bold">Thank you for contacting us!</p>
                <p>We've received your message and will respond within 24 hours.</p>
            </div>
            {% endif %}

            {% if errors %}
            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                <p class="font-bold">Please correct the following errors:</p>
                <ul class="list-disc list-inside">
                    {% for error in errors %}
                    <li>{{ error }}</li>
                    {% endfor %}
                </ul>
            </div>
            {% endif %}
        </div>

        <!-- Contact form -->
        <form
            action="/contact"
            method="post"
            hx-post="/contact"
            hx-target="#contact-result"
            class="bg-white p-6 rounded-lg shadow max-w-2xl"
        >
            <div class="mb-4">
                <label for="name" class="block text-gray-700 font-bold mb-2">
                    Name <span class="text-red-500">*</span>
                </label>
                <input
                    type="text"
                    id="name"
                    name="name"
                    required
                    class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-academy-blue"
                >
            </div>

            <div class="mb-4">
                <label for="email" class="block text-gray-700 font-bold mb-2">
                    Email <span class="text-red-500">*</span>
                </label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    required
                    class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-academy-blue"
                >
            </div>

            <div class="mb-4">
                <label for="subject" class="block text-gray-700 font-bold mb-2">
                    Subject (optional)
                </label>
                <input
                    type="text"
                    id="subject"
                    name="subject"
                    class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-academy-blue"
                >
            </div>

            <div class="mb-4">
                <label for="message" class="block text-gray-700 font-bold mb-2">
                    Message <span class="text-red-500">*</span>
                </label>
                <textarea
                    id="message"
                    name="message"
                    required
                    rows="5"
                    class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-academy-blue"
                ></textarea>
            </div>

            <button
                type="submit"
                class="bg-academy-blue text-white px-6 py-2 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-academy-blue"
            >
                Send Message
            </button>
        </form>
    </section>
</main>

<!-- Footer -->
<footer class="bg-gray-800 text-white py-6 mt-12">
    <div class="max-w-7xl mx-auto px-4 text-center">
        <p>&copy; 2025 {{ academy_name }}. All rights reserved.</p>
    </div>
</footer>
{% endblock %}
```

---

## Step 7: Implement Handlers

### 7.1 Homepage Handler

Edit `src/handlers/homepage.rs`:

```rust
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    academy_name: String,
    tagline: String,
    mission: String,
    programs: Vec<Program>,
    instructors: Vec<Instructor>,
    success: bool,
    errors: Vec<String>,
}

#[derive(Clone)]
struct Program {
    name: String,
    description: String,
}

#[derive(Clone)]
struct Instructor {
    name: String,
    credentials: String,
    photo_path: String,
    bio: String,
}

pub async fn show_homepage() -> Html<String> {
    let template = HomepageTemplate {
        academy_name: "PPST Academy".to_string(),
        tagline: "Excellence in Programming Education".to_string(),
        mission: "We provide world-class programming education...".to_string(),
        programs: vec![
            Program {
                name: "Web Development".to_string(),
                description: "Learn modern web technologies including Rust, HTMX, and more.".to_string(),
            },
            // Add more programs
        ],
        instructors: vec![
            Instructor {
                name: "Jane Smith".to_string(),
                credentials: "Ph.D. in Computer Science".to_string(),
                photo_path: "/static/images/instructors/placeholder.jpg".to_string(),
                bio: "20 years of experience in software engineering.".to_string(),
            },
            // Add more instructors
        ],
        success: false,
        errors: vec![],
    };

    Html(template.render().unwrap())
}
```

### 7.2 Contact Handler

Edit `src/handlers/contact.rs`:

```rust
use axum::{
    extract::Form,
    response::{Html, IntoResponse, Redirect},
    http::{HeaderMap, StatusCode},
};
use crate::models::ContactFormInput;
use crate::storage::save_contact;

pub async fn submit_contact(
    headers: HeaderMap,
    Form(input): Form<ContactFormInput>,
) -> impl IntoResponse {
    // Check if HTMX request
    let is_htmx = headers.get("HX-Request").is_some();

    // Extract user agent and IP (simplified)
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Convert to inquiry and validate
    let inquiry = match input.into_inquiry(None, user_agent) {
        Ok(inq) => inq,
        Err(errors) => {
            // Return validation errors
            if is_htmx {
                let html = format_errors_html(&errors);
                return (StatusCode::BAD_REQUEST, Html(html)).into_response();
            } else {
                // For non-HTMX, redirect back with errors (simplified)
                return Redirect::to("/").into_response();
            }
        }
    };

    // Save to file
    if let Err(e) = save_contact(&inquiry).await {
        eprintln!("Error saving contact: {}", e);
        let html = "<div class='bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded'>An error occurred. Please try again later.</div>";
        return (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response();
    }

    // Return success response
    if is_htmx {
        let html = r#"
            <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                <p class="font-bold">Thank you for contacting us!</p>
                <p>We've received your message and will respond within 24 hours.</p>
            </div>
        "#;
        Html(html).into_response()
    } else {
        Redirect::to("/?success=true").into_response()
    }
}

fn format_errors_html(errors: &[String]) -> String {
    let error_list: String = errors
        .iter()
        .map(|e| format!("<li>{}</li>", e))
        .collect();

    format!(
        r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            <p class="font-bold">Please correct the following errors:</p>
            <ul class="list-disc list-inside">{}</ul>
        </div>"#,
        error_list
    )
}
```

### 7.3 Handlers Module

Edit `src/handlers/mod.rs`:

```rust
pub mod homepage;
pub mod contact;

pub use homepage::show_homepage;
pub use contact::submit_contact;
```

---

## Step 8: Setup Routes

Edit `src/routes.rs`:

```rust
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::handlers::{show_homepage, submit_contact};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(show_homepage))
        .route("/contact", post(submit_contact))
        .nest_service("/static", ServeDir::new("static"))
}
```

---

## Step 9: Main Application

Edit `src/main.rs`:

```rust
mod handlers;
mod models;
mod routes;
mod storage;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ppst_academy=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create router
    let app = routes::create_router();

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
```

---

## Step 10: Run the Application

### Development Mode

Terminal 1 - Build and run Rust application:
```bash
cargo run
```

Terminal 2 - Watch Tailwind CSS:
```bash
npm run css:watch
```

Visit: http://localhost:3000

### Production Build

```bash
# Build Tailwind CSS (minified)
npm run css:build

# Build Rust (release mode)
cargo build --release

# Run production binary
./target/release/ppst-academy
```

---

## Verification Checklist

After completing implementation, verify:

- [ ] Homepage loads at http://localhost:3000
- [ ] All sections visible: header, mission, programs, instructors, contact form, footer
- [ ] Page is responsive (test on mobile/tablet/desktop widths)
- [ ] Contact form submits successfully (check `data/contacts/` for JSON file)
- [ ] Form validation works (try submitting empty form)
- [ ] Success message displays after submission
- [ ] HTMX works (form submits without page reload when JS enabled)
- [ ] Form works without JavaScript (disable JS and test standard POST)
- [ ] Static assets load (CSS, HTMX)
- [ ] Keyboard navigation works (tab through form, submit with Enter)

---

## Troubleshooting

### CSS not loading
- Check Tailwind build output in `static/css/tailwind.css`
- Verify `ServeDir::new("static")` is configured correctly
- Check browser console for 404 errors

### Form submission fails
- Check `data/contacts/` directory exists and is writable
- Review server logs for errors
- Verify form field names match `ContactFormInput` struct

### HTMX not working
- Check `static/js/htmx.min.js` exists
- Verify script tag in base.html
- Check browser console for JavaScript errors
- Test with standard form submission (should still work)

### Templates not rendering
- Verify Askama is in Cargo.toml
- Check template paths match `#[template(path = "...")]`
- Templates must be in `src/templates/` directory
- Run `cargo clean` and rebuild if needed

---

## Next Steps

After completing the quickstart:

1. Add placeholder images for instructors
2. Customize academy content (mission, programs, instructors)
3. Consider moving academy data to TOML config file
4. Add rate limiting for form submissions
5. Implement automated tests
6. Add CSS transitions for better UX
7. Review and deploy to production

For detailed implementation guidance, refer to:
- [research.md](research.md) - Best practices and patterns
- [data-model.md](data-model.md) - Data structures and validation
- [contracts/http-endpoints.md](contracts/http-endpoints.md) - API contracts and examples

---

## Reference Commands

```bash
# Run development server
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Watch Tailwind CSS
npm run css:watch

# Build for production
cargo build --release && npm run css:build

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

**Estimated Implementation Time**: 4-6 hours for a developer familiar with Rust and Axum.

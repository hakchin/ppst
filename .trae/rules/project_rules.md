# Project Rules

## Purpose

- Provide clear, enforceable rules for Trae AI to operate this project consistently
- Derive all operational guidance from `README.md` and current source files

## Preview URL

- <http://127.0.0.1:3000/>
- <http://localhost:3000/>

## Run and Verify

- Start server: `cargo run`
- Open preview: visit `http://127.0.0.1:3000/`
- Expected log: `PPST Academy server listening on http://127.0.0.1:3000`

## Development Commands

- Start server: `cargo run`
- Build release: `cargo build --release` then `./target/release/ppst-academy`
- Tests: `cargo test` (verbose: `cargo test -- --nocapture`)
- Lint: `cargo clippy`
- Format: `cargo fmt` (check: `cargo fmt -- --check`)
- Auto-reload (optional): `cargo install cargo-watch` then `cargo watch -x run`

## Required Checks After Code Changes

- Lint: `cargo clippy`
- Format check: `cargo fmt -- --check`
- Tests: `cargo test` (verbose: `cargo test -- --nocapture`)
- Manual verify in preview: header/logo, menu anchors, HTMX interactions

## Port Troubleshooting

- Find process: `lsof -i :3000`
- Kill process: `kill -9 <PID>`
- Alternative port: set `PORT=<new_port>` when running `cargo run`

## CSS Architecture

- Use 7-1 pattern under `static/css/` (abstracts, base, layout, components, pages, themes, vendors)
- Abstracts folder includes `_variables.css` and `_mixins.css`
- Follow BEM naming: blocks (`.card`), elements (`.card__title`), modifiers (`.card--hoverable`)
- Cascade layers order in `static/css/main.css`: `@layer reset, base, layout, components, pages, vendors;` (static/css/main.css:19)
- No preprocessors; pure CSS; import modules via `static/css/main.css`
- Smooth scroll baseline: `html { scroll-behavior: smooth; }` (static/css/base/_reset.css)

## HTMX Usage

- Local script: `/static/js/htmx.min.js` loaded in base layout (templates/base.html:21)
- Progressive enhancement only; use partial updates where appropriate
- Prefer native anchors for in-page navigation; disable boost per-link when needed (e.g., logo)
- Current body setup: `hx-boost="true" hx-target="#main-content" hx-swap="innerHTML transition:true show:top"` (templates/base.html:23)

## Templates

- Base layout: `templates/base.html`
- Header top anchor target: `<header id="top" ...>` (templates/base.html:29)
- Smooth scroll: global `html { scroll-behavior: smooth; }` (static/css/base/_reset.css)
- Main content container: `<main id="main-content" class="page-transition flex-grow">` (templates/base.html:33)
- Logo anchor: `<a href="#top" class="header__logo" hx-boost="false">` (templates/partials/header.html:3)

## Data Storage

- File-based JSON under `data/contacts/`
- Filename pattern includes ISO timestamp

## Version Control

- Use Jujutsu (jj); common commands are in README

## Manual Testing Checklist

- Server starts and serves `/`
- Homepage loads and styles apply
- Section anchors scroll smoothly
- Contact form validates and submits
- Data saved to `data/contacts/*.json`
- Responsive layout works across breakpoints
- HTMX indicator appears during requests

## Edit Policy

- Prefer editing existing files over creating new ones
- Only create files when absolutely necessary to achieve the goal
- Follow existing patterns and naming; do not introduce preprocessors or new frameworks
- Do not log or expose secrets

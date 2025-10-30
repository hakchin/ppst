# Project Specification

## Project Overview

**Purpose**: Educational website for academy/school promotion (학원 홍보 홈페이지)

**Learning Objectives**:
- Personal learning project for the developer
- Hands-on experience with HTMX-based web development
- Minimal, focused implementation approach

## Technology Stack

### Backend
- **Language**: Rust
- **Web Framework**: Axum
- **Templating Engine**: Askama

### Frontend
- **Server Communication**: HTMX
- **Client-Side Interactivity**: Alpine.js (minimal usage)
- **Styling**: Tailwind CSS

### Build & Development Tools
- **CSS Build Tool**: Tailwind CLI
- **Version Control**: Jujutsu (jj)

## Architecture Principles

### Data Management
- **No Database**: Static content approach, no database system required
- **Content Storage**: Data managed through code/templates

### Development Constraints

**JavaScript Usage**:
- Minimize JavaScript wherever possible
- Rely primarily on HTMX for dynamic interactions
- Use Alpine.js only when client-side state management is absolutely necessary

**Primary Technologies**:
- HTML: Core structure and content
- CSS (Tailwind): Styling and layout
- Server-side rendering via Askama templates

### Design Philosophy
- Keep implementation minimal and focused
- Favor server-side rendering over client-side complexity
- Progressive enhancement approach
- Learning-oriented codebase with clear patterns

## Development Guidelines for AI Assistance

When working on this project, please:
1. Prioritize HTML/CSS solutions over JavaScript
2. Use HTMX attributes for dynamic behavior
3. Keep Alpine.js usage minimal and justified
4. Follow Rust best practices for Axum handlers
5. Maintain clean, educational code structure
6. Avoid introducing database dependencies
7. Use Askama templates for server-side rendering


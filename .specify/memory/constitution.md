<!--
Sync Impact Report:
- Version change: N/A → 1.0.0 (initial constitution)
- New principles added: 5 core principles established
- Templates status: 
  ✅ plan-template.md - constitution check section aligns
  ✅ spec-template.md - requirements alignment verified
  ✅ tasks-template.md - task categorization compatible
- Follow-up TODOs: None
-->

# PPST Academy Website Constitution

## Core Principles

### I. Server-Side First

Development MUST prioritize server-side rendering over client-side complexity. HTML and
CSS solutions MUST be considered before JavaScript alternatives. Askama templates MUST
handle all dynamic content generation. This ensures maintainability, accessibility,
and aligns with the educational goals of the project.

### II. Minimal JavaScript (NON-NEGOTIABLE)

JavaScript usage MUST be minimized and justified. HTMX MUST handle all server
communication and dynamic interactions. Alpine.js MAY only be used when client-side
state management is absolutely necessary and MUST be explicitly justified. No additional
JavaScript frameworks or libraries SHALL be introduced without constitutional amendment.

### III. Static Content Approach

The project MUST maintain a database-free architecture. All content SHALL be managed
through code, templates, and static files. Data persistence through external databases
is PROHIBITED. This constraint ensures simplicity and supports the learning-oriented
nature of the project.

### IV. Educational Code Quality

All code MUST prioritize clarity and educational value over optimization. Clear patterns
MUST be established and consistently followed. Code MUST serve as a learning resource
with well-documented examples of HTMX and Rust web development best practices. Complex
abstractions MUST be avoided in favor of explicit, understandable implementations.

### V. Progressive Enhancement

Features MUST work with basic HTML/CSS first, then enhance with HTMX for dynamic
behavior. The site MUST remain functional without JavaScript. Accessibility MUST be
considered at every development stage. This ensures broad compatibility and demonstrates
modern web development best practices.

## Technology Constraints

### Approved Technology Stack

- **Backend**: Rust with Axum framework (REQUIRED)
- **Templating**: Askama (REQUIRED)
- **Frontend Enhancement**: HTMX (REQUIRED)
- **Styling**: Tailwind CSS (REQUIRED)
- **Minimal Interactivity**: Alpine.js (RESTRICTED USE)
- **Version Control**: Jujutsu (jj) preferred
- **Build Tools**: Tailwind CLI (REQUIRED)

### Prohibited Technologies

- Traditional databases (PostgreSQL, MySQL, SQLite, etc.)
- Heavy JavaScript frameworks (React, Vue, Angular, etc.)
- Complex state management libraries
- Server-side frameworks other than Axum
- CSS frameworks other than Tailwind

## Development Standards

### Code Organization

Source code MUST follow clear separation of concerns with well-defined modules for
handlers, templates, and static assets. File structure MUST support the educational
goals with logical grouping and clear naming conventions. Template organization MUST
follow Askama best practices with reusable components.

### Implementation Guidelines

Every feature implementation MUST demonstrate HTMX capabilities appropriately.
Server responses MUST follow HTMX patterns for partial page updates. Form handling
MUST showcase progressive enhancement principles. Error handling MUST provide clear
feedback without JavaScript dependencies.

## Governance

This constitution supersedes all other development practices and decisions. Changes
require explicit documentation of the rationale and impact assessment. All development
decisions MUST align with the educational objectives and technology constraints defined
herein. Complexity additions MUST be justified against the learning value they provide.

Development guidance is maintained in `.agent/tech_stack_and_architecture.md` for
day-to-day implementation decisions.

**Version**: 1.0.0 | **Ratified**: 2025-10-30 | **Last Amended**: 2025-10-30

<!--
SYNC IMPACT REPORT
==================
Version Change: [NEW] → 1.0.0
Modified Principles: N/A (initial version)
Added Sections:
  - Core Principles (5 principles)
  - Technology Stack Constraints
  - Development Workflow
  - Governance
Removed Sections: N/A
Templates Status:
  ✅ .specify/templates/plan-template.md - Constitution Check section ready for principle validation
  ✅ .specify/templates/spec-template.md - Compatible with principles (prioritized user stories align with simplicity)
  ✅ .specify/templates/tasks-template.md - Task structure supports server-side-first principle
Follow-up TODOs:
  - RATIFICATION_DATE set to today (2025-10-30) as initial adoption
-->

# PPST Academy Website Constitution

## Core Principles

### I. Server-Side First

Server-side rendering and HTML-over-the-wire MUST be the default approach for all features. Client-side JavaScript is permitted only when server-side solutions are demonstrably insufficient. Every client-side script must be justified in code review with a clear explanation of why server-side rendering cannot achieve the same outcome.

**Rationale**: This project is a learning exercise in HTMX-based web development. Minimizing JavaScript ensures the codebase remains focused on mastering server-side rendering patterns and reduces complexity for the educational purpose of the project.

### II. Static Content Architecture

The website MUST NOT introduce database dependencies. All content is managed through templates, code, or static files. Any data persistence requirements must be satisfied through file-based storage or configuration files committed to version control.

**Rationale**: This is a promotional website for an academy with limited dynamic content needs. Avoiding database complexity keeps the project minimal, deployable as a static or simple server application, and aligned with the learning objectives.

### III. HTML/CSS Over JavaScript

HTML semantic elements and CSS (via Tailwind) MUST be exhausted before introducing JavaScript solutions. Interactive behavior should be achieved through:

- HTMX attributes for server communication and partial updates
- CSS pseudo-classes and transitions for visual feedback
- Alpine.js ONLY when stateful client-side interactions are unavoidable

**Rationale**: Mastery of semantic HTML and modern CSS enables building rich, accessible interfaces without framework overhead. This principle enforces exploration of platform capabilities before reaching for JavaScript abstractions.

### IV. Educational Code Quality

Code must prioritize clarity and learning value over clever abstractions. Every component should:

- Follow clear, consistent patterns that demonstrate best practices
- Include comments explaining architectural decisions
- Avoid premature optimization or complex metaprogramming
- Be independently understandable without deep framework knowledge

**Rationale**: The primary purpose of this project is educational. Code should serve as a reference for HTMX patterns, Rust/Axum server architecture, and Askama templating techniques.

### V. Progressive Enhancement

All core functionality MUST work without JavaScript enabled. JavaScript enhancements (HTMX, Alpine.js) should improve user experience but never gate access to essential content or actions.

**Rationale**: Progressive enhancement ensures broad accessibility, forces server-side-first thinking, and provides graceful degradation for users with restricted JavaScript environments.

## Technology Stack Constraints

The following technology choices are fixed for this project:

**Backend**:

- Language: Rust (stable)
- Web Framework: Axum
- Templating: Askama

**Frontend**:

- Server Communication: HTMX
- Client-Side Interactivity: Alpine.js (minimal)
- Styling: Tailwind CSS

**Build & Development**:

- CSS Build: Tailwind CLI
- Version Control: Jujutsu (jj)

**Prohibited**:

- No database systems (SQL or NoSQL)
- No JavaScript frameworks (React, Vue, Svelte, etc.)
- No backend frameworks other than Axum
- No CSS preprocessors other than Tailwind

**Justification for Exceptions**: Any deviation from these constraints requires explicit approval through constitution amendment process with documented learning rationale.

## Development Workflow

### Code Review Gates

All changes must pass:

1. **Principle Compliance**: Verify alignment with Server-Side First, HTML/CSS Over JavaScript, and Static Content Architecture principles
2. **JavaScript Justification**: Any Alpine.js or custom JavaScript must include inline comments explaining why server-side or HTML/CSS approaches were insufficient
3. **Educational Value**: Code should be readable and instructive for future reference

### Testing Requirements

Given the project's educational and promotional nature:

- Manual testing is acceptable for UI/UX features
- Server-side logic (Axum handlers, template rendering) should include unit tests where complexity warrants
- No TDD mandate, but test coverage for critical paths (e.g., form handling, routing) is encouraged

### Documentation Standards

- Template usage patterns should be documented in code comments
- HTMX interaction patterns should be explained when non-obvious
- Architectural decisions should reference relevant principles from this constitution

## Governance

This constitution defines the non-negotiable constraints and principles for the PPST Academy Website project. All development decisions, code reviews, and feature implementations must align with these principles.

### Amendment Process

Amendments require:

1. Documented rationale explaining the change
2. Update to constitution version following semantic versioning
3. Review of dependent templates and documentation for consistency
4. Commit message referencing the amendment

### Version Control

Follows semantic versioning:

- **MAJOR**: Removal or fundamental redefinition of core principles
- **MINOR**: Addition of new principles or substantial guidance expansions
- **PATCH**: Clarifications, wording improvements, non-semantic refinements

### Compliance

- Code reviews MUST verify principle adherence
- JavaScript additions MUST be justified against Principle III
- Database proposals MUST be rejected per Principle II
- Complexity MUST be justified against Principle IV (Educational Code Quality)

**Version**: 1.0.0 | **Ratified**: 2025-10-30 | **Last Amended**: 2025-10-30

# GitHub Copilot Spec-Kit for PPST Academy Website

This directory contains GitHub Copilot instruction files for specification-driven development.

## Overview

The spec-kit provides a structured workflow for feature development:

1. **Constitution Management**: Update project governance and principles
2. **Specification**: Define features from user perspective
3. **Planning**: Create implementation roadmap
4. **Tasks**: Break down into actionable work items

## Files

- **constitution.md**: Instructions for updating project constitution
- **specify.md**: Instructions for creating feature specifications
- **plan.md**: Instructions for generating implementation plans
- **tasks.md**: Instructions for creating task breakdowns

## Usage in VS Code

Use `@workspace` mention in GitHub Copilot Chat with natural language:

### Update Constitution

```text
@workspace Update the project constitution: add rate limiting for contact forms
```

### Create Specification

```text
@workspace Create specification for: homepage with academy information and contact form
```

### Generate Implementation Plan

```text
@workspace Create plan for: spec-001-homepage-contact
```

### Create Task Breakdown

```text
@workspace Create tasks for: plan-001-homepage-contact
```

## Workflow Example

**Step 1**: Create specification

```text
@workspace Create specification for: student registration system with email verification
```

Result: `.specify/specs/spec-002-student-registration.md`

**Step 2**: Generate plan

```text
@workspace Create plan for: spec-002-student-registration
```

Result: `.specify/plans/plan-002-student-registration.md`

**Step 3**: Break down tasks

```text
@workspace Create tasks for: plan-002-student-registration
```

Result: `.specify/tasks/tasks-002-student-registration.md`

**Step 4**: Implement

Follow tasks in sequential order, checking off completion.

## Project Structure

```text
.github/copilot/          # Copilot instruction files (this directory)
.copilot/
  ├── README.md           # Spec-kit documentation
  ├── memory/
  │   └── constitution.md # Project governance
  ├── templates/          # Document templates
  └── scripts/            # Helper scripts
specs/                    # Feature specifications
  └── {number}-{name}/    # Individual spec directories
```

## Constitution Principles

All work must comply with:

1. **Server-Side First**: HTML-over-the-wire, minimal JavaScript
2. **Static Content Architecture**: No databases, file-based storage
3. **HTML/CSS Over JavaScript**: Avoid Alpine.js unless necessary
4. **Educational Code Quality**: Clear, instructive patterns
5. **Progressive Enhancement**: Works without JavaScript

## Technology Stack

**Approved**:
- Rust + Axum (backend)
- Askama (templating)
- HTMX (enhancement only)
- Tailwind CSS (styling)
- Tailwind CLI standalone (build)

**Prohibited**:
- Node.js/npm
- Databases (PostgreSQL, MySQL, SQLite, etc.)
- JavaScript frameworks (React, Vue, Angular)
- Alpine.js (unless absolutely necessary and justified)

## Tips

- Be specific in requests: "Create specification for: user authentication with email verification"
- Reference existing specs/plans by ID: "Create plan for: spec-001-homepage-contact"
- Update constitution for project-wide changes: "Update constitution: add security requirements"
- Check `.specify/` directory for existing work before creating new specs

## Related Files

- Constitution: `.copilot/memory/constitution.md`
- Spec-Kit Documentation: `.copilot/README.md`
- Templates: `.copilot/templates/`

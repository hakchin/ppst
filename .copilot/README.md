# GitHub Copilot Spec-Kit for PPST Academy Website

This directory contains the specification-driven development framework for working with GitHub Copilot.

## Overview

The spec-kit provides a structured workflow for feature development using GitHub Copilot CLI and Chat:

1. **Constitution Management**: Update project governance and principles
2. **Specification**: Define features from user perspective
3. **Planning**: Create implementation roadmap
4. **Tasks**: Break down into actionable work items

## Directory Structure

```text
.copilot/
├── README.md              # This file
├── memory/
│   └── constitution.md    # Project governance and principles
├── templates/             # Document templates
│   ├── spec-template.md
│   ├── plan-template.md
│   └── tasks-template.md
└── scripts/               # Helper scripts
    └── bash/
```

## Usage with GitHub Copilot

### In VS Code with Copilot Chat

Use `@workspace` mention in GitHub Copilot Chat:

```text
@workspace Create specification for: homepage with contact form
```

Reference the instruction files in `.github/copilot/` directory.

### In Terminal with Copilot CLI

```bash
# Create a new specification
github-copilot-cli "Create a specification for homepage with contact form following .copilot/templates/spec-template.md"

# Generate implementation plan
github-copilot-cli "Create implementation plan for spec-001-homepage following .copilot/templates/plan-template.md"

# Break down into tasks
github-copilot-cli "Break down plan-001-homepage into actionable tasks following .copilot/templates/tasks-template.md"
```

## Workflow

**Step 1: Create Specification**

Define the feature from a user perspective using `specs/{number}-{name}/spec.md`.

**Step 2: Generate Implementation Plan**

Create a technical roadmap using `specs/{number}-{name}/plan.md`.

**Step 3: Break Down Tasks**

Create actionable work items using `specs/{number}-{name}/tasks.md`.

**Step 4: Implement**

Follow tasks in sequential order, checking off completion.

## Constitution Principles

All work must comply with the project constitution (`.copilot/memory/constitution.md`):

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

## Project Structure

```text
specs/
├── 001-homepage/
│   ├── spec.md           # Feature specification
│   ├── plan.md           # Implementation plan
│   ├── tasks.md          # Task breakdown
│   ├── data-model.md     # Data structures
│   ├── research.md       # Research notes
│   ├── quickstart.md     # Quick reference
│   ├── checklists/       # Validation checklists
│   └── contracts/        # API contracts
└── 002-next-feature/
```

## Tips

- Be specific in requests: "Create specification for: user authentication with email verification"
- Reference existing specs by ID: "Create plan for: spec-001-homepage"
- Update constitution for project-wide changes
- Check `specs/` directory for existing work before creating new specs
- Use templates as guides for consistency

## Related Files

- Constitution: `.copilot/memory/constitution.md`
- Copilot Instructions: `.github/copilot/`
- Templates: `.copilot/templates/`
- Current Specs: `specs/`

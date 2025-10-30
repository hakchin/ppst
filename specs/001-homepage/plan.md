# Implementation Plan: Academy Homepage

**Branch**: `001-homepage` | **Date**: 2025-10-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-homepage/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Create a responsive homepage for PPST Academy featuring academy information (mission, programs, instructors) and a contact form. The implementation will follow the server-side-first approach using Rust/Axum for the backend, Askama for templating, and HTMX for enhanced interactivity while maintaining progressive enhancement principles. All functionality must work without JavaScript, with HTMX providing improved UX for users with JS enabled.

## Technical Context

**Language/Version**: Rust (stable channel, latest)
**Primary Dependencies**: Axum (web framework), Askama (templating), Tower (middleware), Tokio (async runtime)
**Storage**: File-based storage for contact form submissions (JSON or structured text files)
**Testing**: cargo test (unit tests), manual testing for UI/UX validation
**Target Platform**: Web server (Linux/macOS/Windows), targeting modern browsers (Chrome, Firefox, Safari)
**Project Type**: Single web application with server-side rendering
**Performance Goals**: <3 second page load on broadband, <200ms server response time for form submissions
**Constraints**: No database dependencies, JavaScript must not be required for core functionality, WCAG 2.1 Level AA accessibility
**Scale/Scope**: Small promotional website, expected <1000 concurrent users, single homepage with 4-5 content sections

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Server-Side First ✅

- **Compliance**: All core functionality (displaying academy info, form submission) will be handled server-side using Axum handlers and Askama templates
- **HTMX Usage**: HTMX will be used only for enhancements (smooth form submission, partial page updates) but NOT required for core functionality
- **Verification**: Form submission will work with standard HTTP POST; HTMX provides enhanced UX but is not mandatory

### Principle II: Static Content Architecture ✅

- **Compliance**: No database dependencies - academy information stored in templates/code
- **Storage**: Contact form submissions saved to local JSON/text files
- **Verification**: All content manageable through version-controlled files

### Principle III: HTML/CSS Over JavaScript ✅

- **Compliance**:
  - Responsive design via Tailwind CSS
  - Navigation via anchor links (no JS required)
  - Form validation using HTML5 attributes + server-side validation
  - HTMX used only for enhanced form submission UX
  - Alpine.js: NOT NEEDED for this feature
- **Verification**: Page fully functional with JavaScript disabled (FR-015)

### Principle IV: Educational Code Quality ✅

- **Compliance**:
  - Clear separation: routes → handlers → templates
  - Inline comments explaining HTMX attributes and Axum patterns
  - Simple, direct code structure without over-engineering
- **Verification**: Code should serve as reference for Axum + HTMX patterns

### Principle V: Progressive Enhancement ✅

- **Compliance**:
  - Base: Standard HTML forms, server-rendered content
  - Enhancement Layer 1: CSS-based responsive design and visual feedback
  - Enhancement Layer 2: HTMX for improved form submission (no page reload)
- **Verification**: All user stories testable with JS disabled (SC-006)

### Technology Stack Compliance ✅

- **Backend**: Rust + Axum + Askama ✅
- **Frontend**: HTMX + Tailwind CSS ✅
- **No Prohibited Technologies**: No databases, no React/Vue/Svelte, no other backends ✅

### Overall Assessment: **PASS** ✅

All constitutional principles are satisfied. No complexity violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Application entry point, server setup
├── routes.rs            # Route definitions and handler registration
├── handlers/
│   ├── mod.rs
│   ├── homepage.rs      # Homepage rendering handler
│   └── contact.rs       # Contact form submission handler
├── templates/
│   ├── base.html        # Base template with common layout
│   ├── homepage.html    # Main homepage template
│   └── partials/
│       ├── header.html  # Header section (logo, nav)
│       ├── mission.html # Mission statement section
│       ├── programs.html # Programs section
│       ├── instructors.html # Instructors section
│       └── contact_form.html # Contact form component
├── models/
│   ├── mod.rs
│   └── contact.rs       # Contact inquiry data structure
└── storage/
    ├── mod.rs
    └── file_store.rs    # File-based storage for contact submissions

static/
├── css/
│   └── tailwind.css     # Generated Tailwind CSS
└── js/
    └── htmx.min.js      # HTMX library (optional enhancement)

data/
└── contacts/            # Directory for storing contact form submissions
    └── .gitkeep

tests/
├── integration/
│   └── homepage_test.rs # Integration tests for homepage routes
└── unit/
    ├── contact_test.rs  # Unit tests for contact model
    └── storage_test.rs  # Unit tests for file storage

Cargo.toml               # Rust dependencies
tailwind.config.js       # Tailwind configuration
```

**Structure Decision**: Single Rust web application structure. All functionality is server-rendered with Axum handling HTTP requests, Askama rendering templates, and static files served for CSS/JS enhancements. File-based storage eliminates need for database infrastructure.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitutional violations. This feature fully complies with all principles.

---

## Phase 0: Research ✅

**Status**: Complete

**Artifacts Generated**:
- [research.md](research.md) - Technology stack research and best practices

**Key Findings**:
- All technology choices confirmed by constitution
- Axum + Askama + HTMX + Tailwind patterns well-documented
- File-based storage sufficient for contact form submissions
- Progressive enhancement strategy defined
- WCAG 2.1 Level AA accessibility requirements identified

**No Clarifications Required**: All technical decisions are clear.

---

## Phase 1: Design & Contracts ✅

**Status**: Complete

**Artifacts Generated**:
- [data-model.md](data-model.md) - Entity definitions and validation rules
- [contracts/http-endpoints.md](contracts/http-endpoints.md) - HTTP API contracts
- [quickstart.md](quickstart.md) - Implementation guide
- CLAUDE.md - Updated agent context

**Key Deliverables**:

### Data Model
- `ContactInquiry` entity with validation
- `AcademyInfo` structure for static content
- File-based JSON storage specification

### HTTP Endpoints
- `GET /` - Homepage rendering
- `POST /contact` - Form submission (supports both standard and HTMX)
- `GET /static/*` - Static asset serving

### Implementation Strategy
- Server-side rendering with Askama templates
- Progressive enhancement with HTMX
- File-based persistence (no database)
- Comprehensive validation (server-side)

---

## Constitution Check - Post-Design Re-evaluation ✅

After completing research and design phases, re-evaluating constitutional compliance:

### Principle I: Server-Side First ✅

**Design Compliance**:
- All rendering happens server-side (Askama templates)
- Form submissions handled by Axum handlers
- HTMX only enhances UX, doesn't replace server logic
- Endpoints work identically with or without HTMX (detected via `HX-Request` header)

**Verification**: HTTP contracts show dual-mode support (standard POST + HTMX).

### Principle II: Static Content Architecture ✅

**Design Compliance**:
- Academy information: hard-coded in Rust structs or TOML config
- Contact submissions: JSON files in `data/contacts/` directory
- No database dependencies in Cargo.toml
- All storage is file-based or in-memory

**Verification**: Data model uses `std::fs` for persistence, no SQL/NoSQL dependencies.

### Principle III: HTML/CSS Over JavaScript ✅

**Design Compliance**:
- Responsive design: Tailwind utility classes (no JS required)
- Form validation: HTML5 attributes + server-side validation
- Navigation: Standard anchor links
- HTMX: Optional enhancement layer only
- Alpine.js: Not needed for this feature

**Verification**: Quickstart guide shows page works fully without JavaScript (FR-015).

### Principle IV: Educational Code Quality ✅

**Design Compliance**:
- Clear code structure: handlers → models → storage → templates
- Simple validation logic (manual, not abstracted)
- Extensive inline comments planned in quickstart
- Pattern demonstrations for Rust/Axum/HTMX integration

**Verification**: Quickstart provides educational code samples with explanations.

### Principle V: Progressive Enhancement ✅

**Design Compliance**:
- **Base layer**: HTML forms, server rendering, standard HTTP POST
- **Layer 1**: CSS-based responsive design and visual feedback
- **Layer 2**: HTMX for enhanced form submission (no page reload)

**Verification**: HTTP contracts define separate response paths for standard vs. HTMX requests.

### Technology Stack Compliance ✅

- ✅ Rust (stable)
- ✅ Axum web framework
- ✅ Askama templating
- ✅ HTMX for enhancements
- ✅ Tailwind CSS for styling
- ✅ No databases
- ✅ No prohibited technologies

### Final Assessment: **FULL COMPLIANCE** ✅

The design artifacts confirm that all constitutional principles are satisfied. No violations identified. Implementation can proceed with confidence.

---

## Next Steps

### Ready for Implementation

This planning phase is complete. The following artifacts are ready for use:

1. **[plan.md](plan.md)** (this file) - Technical context and structure
2. **[research.md](research.md)** - Best practices and patterns
3. **[data-model.md](data-model.md)** - Entity definitions and validation
4. **[contracts/http-endpoints.md](contracts/http-endpoints.md)** - API contracts
5. **[quickstart.md](quickstart.md)** - Step-by-step implementation guide

### Task Generation

To generate implementation tasks, run:

```bash
/speckit.tasks
```

This will create `tasks.md` with organized, dependency-ordered tasks for implementing the homepage feature.

### Implementation

Once tasks are generated, begin implementation by following the quickstart guide and completing tasks in priority order (P1 → P2 → P3).

---

**Planning Phase Complete** ✅

**Branch**: `001-homepage`
**Status**: Ready for task generation and implementation
**Estimated Effort**: 4-6 hours for experienced Rust/Axum developer

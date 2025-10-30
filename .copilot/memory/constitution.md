# PPST Academy Website - Project Constitution

Version: 1.0.0  
Last Updated: 2025-11-06

## Purpose

This constitution defines the non-negotiable principles, constraints, and guidelines for the PPST Academy Website project. All specifications, plans, and implementations must comply with these rules.

## Core Principles

### I. Server-Side First

**Philosophy**: Generate HTML on the server; send complete markup to the client.

**Requirements**:
- All content MUST be rendered via Askama templates on the server
- JavaScript is for enhancement only, never for core functionality
- Pages must work with JavaScript disabled
- Client-side logic requires explicit justification in specifications

**Rationale**: Educational clarity, SEO, accessibility, performance

### II. Static Content Architecture

**Philosophy**: No database dependencies; embrace simplicity.

**Requirements**:
- NO database systems (PostgreSQL, MySQL, SQLite, Redis, etc.)
- Data storage uses file-based JSON in `data/` directory
- File naming convention: `data/{entity}/{YYYY-MM-DDTHH-MM-SS-mmmZ}.json`
- Configuration in version control
- Read-only data can be embedded in code

**Rationale**: Educational focus, deployment simplicity, version control

### III. HTML/CSS Over JavaScript

**Philosophy**: Maximize what HTML and CSS can do before reaching for JavaScript.

**Requirements**:
- NO Alpine.js unless absolutely necessary and justified
- HTMX only for dynamic interactions that enhance UX
- Forms must work with native HTML form submission
- Styling via Tailwind CSS utility classes
- Full functionality without JavaScript required

**Rationale**: Educational patterns, progressive enhancement, maintainability

### IV. Educational Code Quality

**Philosophy**: Code is teaching material.

**Requirements**:
- Clear, instructive code patterns
- Well-documented examples
- Avoid complex abstractions
- Favor explicitness over cleverness
- Comments explain "why" not "what"

**Rationale**: Primary audience is learners; code quality matters

### V. Progressive Enhancement

**Philosophy**: Build from the baseline up.

**Requirements**:
- Layer 1: HTML + CSS (base functionality)
- Layer 2: HTMX (enhancement only)
- Layer 3: Minimal JavaScript (if justified)
- Each layer degrades gracefully
- Test with JavaScript disabled

**Rationale**: Accessibility, resilience, educational value

## Technology Stack Constraints

### Approved Technologies

**Backend**:
- Rust (stable)
- Axum (web framework)
- Askama (HTML templating)
- Serde (JSON serialization)
- Tokio (async runtime)

**Frontend**:
- HTML5
- CSS3 + Tailwind CSS
- HTMX (for dynamic interactions)
- Minimal vanilla JavaScript (if justified)

**Build Tools**:
- Cargo (Rust package manager)
- Tailwind CLI standalone (CSS build)

**Development**:
- Git (version control)
- VS Code (recommended editor)
- GitHub Copilot (AI assistance)

### Prohibited Technologies

**Databases**:
- ❌ PostgreSQL, MySQL, SQLite
- ❌ Redis, MongoDB, any database system
- ❌ ORMs, query builders

**JavaScript**:
- ❌ Node.js, npm, yarn, pnpm
- ❌ React, Vue, Angular, Svelte
- ❌ Alpine.js (unless explicitly justified)
- ❌ jQuery, Lodash, utility libraries
- ❌ Webpack, Vite, Rollup, bundlers

**Backend**:
- ❌ Python, PHP, Ruby, Go, Java
- ❌ Express, Django, Rails, Laravel

### Justification Process

If a prohibited technology is absolutely necessary:

1. Document in specification why no approved alternative works
2. Explain educational value or critical need
3. Propose minimal usage scope
4. Get explicit approval in plan review

## Development Workflow

### Specification-Driven Development

1. **Specification Phase**: Define feature from user perspective
   - Focus on business value and user needs
   - No implementation details
   - Clear acceptance criteria

2. **Planning Phase**: Create technical implementation plan
   - Architecture decisions
   - File structure
   - Code patterns
   - Constitution compliance verification

3. **Tasks Phase**: Break down into actionable work
   - Atomic, testable tasks
   - Clear dependencies
   - Effort estimates

4. **Implementation Phase**: Execute tasks
   - Follow constitution principles
   - Pass constitution gates
   - Document as you go

### Constitution Gate Checks

Every implementation phase must pass:

- ✅ **Server-Side First**: No unjustified client-side logic
- ✅ **Static Content**: No database dependencies
- ✅ **HTML/CSS First**: JavaScript minimized and justified
- ✅ **Educational Quality**: Clear, instructive code
- ✅ **Progressive Enhancement**: Works without JavaScript

Failed gate check = implementation rejected

## File Structure Standards

### Project Organization

```text
ppst/
├── src/
│   ├── main.rs              # Application entry point
│   ├── routes.rs            # Route definitions
│   ├── handlers/            # Request handlers
│   │   └── {feature}.rs
│   ├── models/              # Data structures
│   │   └── {entity}.rs
│   └── utils/               # Shared utilities
├── templates/               # Askama templates
│   ├── base.html           # Base layout
│   ├── {page}.html         # Page templates
│   └── partials/           # Reusable components
├── static/                  # Static assets
│   ├── css/                # Compiled CSS
│   └── images/             # Images
├── data/                    # File-based storage
│   └── {entity}/           # Entity data
├── specs/                   # Feature specifications
│   └── {number}-{name}/    # Spec directory
└── .copilot/               # Copilot spec-kit
```

### Data Storage Conventions

**Contact Form Submissions**:
```text
data/contacts/2024-01-15T14-30-00-123Z.json
```

**Configuration Files**:
```text
data/config/academy.json
data/config/courses.json
```

**File Naming**:
- ISO 8601 timestamps for time-series data
- Kebab-case for configuration files
- JSON format for all data

## Governance

### Constitution Amendments

**Process**:
1. Propose change with rationale
2. Document impact on existing specs/plans
3. Update version number (semantic versioning)
4. Sync instruction files in `.github/copilot/`
5. Update templates in `.copilot/templates/`

**Version Semantics**:
- **MAJOR (x.0.0)**: Remove or redefine core principles
- **MINOR (0.x.0)**: Add new principles or expand guidance
- **PATCH (0.0.x)**: Clarifications, wording, typos

### Review Authority

Constitution changes require:
- Technical justification
- Educational value assessment
- Impact analysis on existing work
- Backward compatibility consideration

## Compliance

### Specification Checklist

Every specification must verify:
- [ ] No database dependencies
- [ ] Server-side rendering approach
- [ ] HTML/CSS first design
- [ ] JavaScript usage justified (if any)
- [ ] Progressive enhancement verified
- [ ] File-based storage defined
- [ ] Educational code patterns

### Plan Checklist

Every implementation plan must verify:
- [ ] Askama templates for all views
- [ ] Axum handlers for all routes
- [ ] File-based data storage
- [ ] HTMX for enhancements (if needed)
- [ ] No Alpine.js (unless justified)
- [ ] Constitution gates per phase
- [ ] Testing without JavaScript

### Code Review Checklist

Every implementation must verify:
- [ ] Works with JavaScript disabled
- [ ] Clear, educational code quality
- [ ] Proper error handling
- [ ] Accessibility standards
- [ ] Mobile responsive design
- [ ] File storage follows conventions

## References

- Technology Stack Details: `.github/copilot/README.md`
- Specification Template: `.copilot/templates/spec-template.md`
- Plan Template: `.copilot/templates/plan-template.md`
- Tasks Template: `.copilot/templates/tasks-template.md`

---

**Remember**: These principles are non-negotiable. They define what makes this project educational, maintainable, and aligned with our values. When in doubt, ask: "Does this serve the learner?"

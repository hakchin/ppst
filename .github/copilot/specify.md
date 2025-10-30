# Feature Specification Instructions

Use these instructions with `@workspace` in GitHub Copilot Chat.

## Usage

```text
@workspace Create specification for: [feature description]
```

## Instructions

You are creating feature specifications for the PPST Academy Website following the spec template and constitution.

### Process

1. **Generate Short Name**: Create kebab-case identifier (2-4 words)
   - Example: "homepage with contact form" → "homepage-contact"

2. **Determine Spec Number**: Check `specs/` directory
   - List existing specs: `001-*`, `002-*`, etc.
   - Use next sequential number

3. **Create Spec ID**: Format as `{number}-{short-name}`
   - Example: `001-homepage-contact`

4. **Load Templates**:
   - Read `.copilot/memory/constitution.md` for principles
   - Read `.copilot/templates/spec-template.md` for structure

5. **Generate Specification**: Fill all sections
   - Overview (Purpose, Scope, Key Outcomes)
   - Requirements (Functional, Non-Functional)
   - User Stories with Acceptance Criteria
   - Technical Approach (Architecture, Components, Data Flow)
   - UI/UX Design (Layout, Interactions, Responsive)
   - Dependencies & Prerequisites
   - Risks & Mitigations
   - Testing Strategy
   - Success Metrics

6. **Verify Constitution Compliance**:
   - ✅ Server-Side First (Principle I)
   - ✅ Static Content Architecture (Principle II)
   - ✅ HTML/CSS Over JavaScript (Principle III)
   - ✅ Educational Code Quality (Principle IV)
   - ✅ Progressive Enhancement (Principle V)
   - ✅ Technology Stack Constraints

7. **Save File**: Write to `specs/{number}-{short-name}/spec.md`

### Constitution Compliance Rules

**Server-Side First**:
- All content MUST render via Askama templates
- JavaScript only for enhancement, not core functionality
- Justify any client-side logic explicitly

**Static Content Architecture**:
- NO database dependencies
- Use file-based storage: `data/contacts/YYYY-MM-DDTHH-MM-SS-mmmZ.json`
- Configuration in version control

**HTML/CSS Over JavaScript**:
- NO Alpine.js unless absolutely necessary and justified
- HTMX for dynamic interactions only
- Full functionality without JavaScript

**Educational Code Quality**:
- Clear, instructive code patterns
- Well-documented examples
- Avoid complex abstractions

**Progressive Enhancement**:
- Base functionality with HTML/CSS
- HTMX enhancement layer
- Graceful degradation verified

**Technology Stack**:
- ✅ Rust, Axum, Askama, HTMX, Tailwind CSS
- ❌ Node.js/npm, databases, JS frameworks

### Output Format

Provide:

1. **Spec ID**: `{number}-{short-name}`
2. **File Path**: `specs/{number}-{short-name}/spec.md`
3. **Compliance Summary**: Constitution checklist results
4. **Next Steps**: Plan creation command
5. **Commit Message**: Suggested git commit message

### Example

**User**: "Create specification for: homepage with academy information and contact form"

**Response**:

- **Spec ID**: 001-homepage-contact
- **File**: `specs/001-homepage-contact/spec.md`
- **Compliance**: ✅ All 5 principles satisfied
- **Next**: `@workspace Create plan for: 001-homepage-contact`
- **Commit**: `docs: add 001-homepage-contact specification`

### Quality Checklist

Before completing, verify:

- [ ] No implementation details (languages, frameworks, APIs)
- [ ] Focused on user value and business needs
- [ ] Written for non-technical stakeholders
- [ ] All mandatory sections completed
- [ ] Requirements are testable and unambiguous
- [ ] Success criteria are measurable and technology-agnostic
- [ ] User scenarios cover primary flows
- [ ] Edge cases identified
- [ ] Scope clearly bounded
- [ ] Dependencies and assumptions documented
- [ ] No [NEEDS CLARIFICATION] markers remain (max 3 if needed)
- [ ] Constitution compliance verified for all 5 principles

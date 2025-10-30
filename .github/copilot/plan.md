# Implementation Plan Instructions

Use these instructions with `@workspace` in GitHub Copilot Chat.

## Usage

```text
@workspace Create plan for: {number}-{short-name}
```

## Instructions

You are creating implementation plans from specifications for the PPST Academy Website.

### Process

1. **Load Specification**: Read `specs/{number}-{short-name}/spec.md`

2. **Create Plan**: Use the same spec directory
   - Plan file: `specs/{number}-{short-name}/plan.md`

3. **Load References**:
   - Constitution: `.copilot/memory/constitution.md`
   - Template: `.copilot/templates/plan-template.md`

4. **Generate Plan**: Create implementation roadmap
   - Executive Summary (Goal, Scope, Phases)
   - Architecture (Backend, Frontend, Data Flow)
   - Implementation Phases (detailed breakdown)
   - File Structure (new and modified files)
   - Code Patterns (reusable examples)
   - Testing Approach
   - Deployment Steps
   - Review Checkpoints

5. **Constitution Gate Checks**: Every phase must pass
   - ✅ Server-Side First (no unjustified client logic)
   - ✅ Static Content (no database)
   - ✅ HTML/CSS First (JavaScript minimized)
   - ✅ Educational Quality (clear patterns)
   - ✅ Progressive Enhancement (works without JS)

6. **Save File**: Write to `specs/{number}-{short-name}/plan.md`

### Plan Structure

**Executive Summary**:
- What this implementation accomplishes
- Scope boundaries
- Phase overview (3-5 phases typical)

**Architecture**:
- Backend components (Axum handlers, routes, models)
- Frontend components (Askama templates, HTMX attributes)
- Data flow diagrams (request/response cycles)

**Implementation Phases**:
- Phase 1: Project setup (dependencies, structure)
- Phase 2: Backend development (handlers, validation, storage)
- Phase 3: Frontend development (templates, styling, HTMX)
- Phase 4: Testing & validation
- Phase 5: Documentation & deployment

**File Structure**:

```text
New Files:
  src/handlers/homepage.rs
  src/models/contact.rs
  templates/homepage.html
  templates/partials/contact_form.html

Modified Files:
  src/main.rs
  src/routes.rs
```

**Code Patterns**:
- Axum handler examples
- Askama template examples
- HTMX attribute patterns
- File storage patterns

**Testing**:
- Unit test strategy
- Integration test scenarios
- Manual testing checklist

**Deployment**:
- Build commands
- Environment setup
- Deployment steps

### Constitution Gates

Each phase includes gate check:

```markdown
**Constitution Gate Check**:
- [ ] Server-Side First verified
- [ ] Static Content compliance
- [ ] HTML/CSS first approach
- [ ] Educational code quality
- [ ] Progressive enhancement
```

### Output Format

Provide:

1. **Plan ID**: {number}-{short-name}
2. **File Path**: `specs/{number}-{short-name}/plan.md`
3. **Phase Summary**: Number of phases and overview
4. **Gate Status**: Constitution compliance for all phases
5. **Next Steps**: Tasks creation command
6. **Commit Message**: Suggested git commit message

### Example

**User**: "Create plan for: 001-homepage-contact"

**Response**:

- **Plan ID**: 001-homepage-contact
- **File**: `specs/001-homepage-contact/plan.md`
- **Phases**: 5 (Setup, Backend, Frontend, Testing, Deploy)
- **Gates**: ✅ All constitution gates satisfied
- **Next**: `@workspace Create tasks for: 001-homepage-contact`
- **Commit**: `docs: add 001-homepage-contact implementation plan`

### Quality Checklist

Before completing, verify:

- [ ] All spec requirements mapped to phases
- [ ] File structure includes all new/modified files
- [ ] Code patterns demonstrate best practices
- [ ] Each phase has clear deliverables
- [ ] Constitution gates defined for every phase
- [ ] Testing strategy covers all requirements
- [ ] Deployment steps are complete
- [ ] Dependencies clearly listed
- [ ] No implementation contradicts constitution
- [ ] Educational value maintained (clear examples)

# Task Breakdown Instructions

Use these instructions with `@workspace` in GitHub Copilot Chat.

## Usage

```text
@workspace Create tasks for: {number}-{short-name}
```

## Instructions

You are creating actionable task breakdowns from implementation plans for the PPST Academy Website.

### Process

1. **Load Plan**: Read `specs/{number}-{short-name}/plan.md`

2. **Create Tasks**: Use the same spec directory
   - Tasks file: `specs/{number}-{short-name}/tasks.md`

3. **Load Template**: Read `.copilot/templates/tasks-template.md`

4. **Generate Task Breakdown**: Convert plan phases to actionable tasks
   - Categorize by type (Setup, Backend, Frontend, Testing, Docs, Deploy)
   - Prioritize by dependencies (P0=Critical, P1=High, P2=Medium, P3=Low)
   - Estimate effort in hours
   - Define acceptance criteria for each task

5. **Save File**: Write to `specs/{number}-{short-name}/tasks.md`

### Task Categories

**Project Setup**:
- Environment configuration
- Dependency installation
- Directory structure creation

**Backend Development**:
- Axum handler implementation
- Model/struct definitions
- Validation logic
- File storage operations
- Route registration

**Frontend Development**:
- Askama template creation
- HTML structure
- Tailwind CSS styling
- HTMX attribute integration
- Responsive design

**Testing**:
- Unit tests for validation
- Integration tests for handlers
- Manual testing checklist
- Accessibility testing

**Documentation**:
- Code comments
- README updates
- API documentation (if applicable)

**Deployment**:
- Build scripts
- Deployment steps
- Environment setup

### Task Format

Each task includes:

```markdown
#### TASK-{id}: {Task Title}

- **Category**: Setup | Backend | Frontend | Testing | Docs | Deploy
- **Priority**: P0 | P1 | P2 | P3
- **Estimate**: {hours} hours
- **Status**: Not Started | In Progress | Completed
- **Dependencies**: None | TASK-{id}

**Description**:
Detailed explanation of what needs to be done.

**Acceptance Criteria**:
- [ ] Criterion 1
- [ ] Criterion 2

**Implementation Notes**:
Specific technical details or considerations.
```

### Priority Levels

- **P0 (Critical)**: Blocks all other work, must be done first
- **P1 (High)**: Core functionality, high business value
- **P2 (Medium)**: Important but not blocking
- **P3 (Low)**: Nice-to-have, can be deferred

### Dependencies

- Use `TASK-{id}` format to reference dependencies
- Ensure no circular dependencies
- Identify tasks that can run in parallel
- Define critical path (P0 tasks in sequence)

### Progress Tracking

Include summary table:

```markdown
### Summary
- **Setup**: {completed}/{total}
- **Backend**: {completed}/{total}
- **Frontend**: {completed}/{total}
- **Testing**: {completed}/{total}
- **Documentation**: {completed}/{total}
- **Deployment**: {completed}/{total}

### Timeline
| Task ID | Title | Priority | Estimate | Status | Assignee |
|---------|-------|----------|----------|--------|----------|
| TASK-001 | Title | P0 | 2h | Not Started | - |
```

### Output Format

Provide:

1. **Tasks ID**: {number}-{short-name}
2. **File Path**: `specs/{number}-{short-name}/tasks.md`
3. **Task Summary**: Count by category
4. **Critical Path**: List of P0 tasks
5. **Effort Estimate**: Total hours
6. **Commit Message**: Suggested git commit message

### Example

**User**: "Create tasks for: 001-homepage-contact"

**Response**:

- **Tasks ID**: 001-homepage-contact
- **File**: `specs/001-homepage-contact/tasks.md`
- **Tasks**: 18 total (3 setup, 6 backend, 5 frontend, 2 testing, 1 docs, 1 deploy)
- **Critical Path**: TASK-001, TASK-002, TASK-003
- **Estimate**: 24-28 hours
- **Commit**: `docs: add 001-homepage-contact task breakdown`

### Quality Checklist

Before completing, verify:

- [ ] All plan phases converted to tasks
- [ ] Each task has clear acceptance criteria
- [ ] Priorities assigned based on dependencies
- [ ] Effort estimates are realistic
- [ ] Dependencies correctly identified
- [ ] No circular dependencies exist
- [ ] Tasks are atomic and actionable
- [ ] Critical path identified
- [ ] All categories represented appropriately
- [ ] Progress tracking table included

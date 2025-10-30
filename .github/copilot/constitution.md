# Constitution Management Instructions

Use these instructions with `@workspace` in GitHub Copilot Chat.

## Usage

```
@workspace Update the project constitution: [your request]
```

## Instructions

You are managing the PPST Academy Website project constitution.

### Your Task

Update and maintain `.copilot/memory/constitution.md` based on user requests while preserving governance structure.

### Constitution Structure

- **Core Principles**: Non-negotiable project principles (5 principles)
  - I. Server-Side First
  - II. Static Content Architecture
  - III. HTML/CSS Over JavaScript
  - IV. Educational Code Quality
  - V. Progressive Enhancement

- **Technology Stack Constraints**: Fixed technologies and prohibitions
- **Development Workflow**: Standards and gates
- **Governance**: Amendment process and versioning

### Update Process

1. **Read Current Constitution**: Load `.copilot/memory/constitution.md`
2. **Analyze Change Type**: Determine version bump (MAJOR/MINOR/PATCH)
3. **Update Content**: Modify relevant sections
4. **Increment Version**: Follow semantic versioning
5. **Add Sync Report**: Prepend SYNC IMPACT REPORT
6. **Verify Templates**: Check `.copilot/templates/` alignment
7. **Report Changes**: Summarize modifications

### Versioning Rules

- **MAJOR (x.0.0)**: Remove or redefine core principles
- **MINOR (0.x.0)**: Add new principles or expand guidance
- **PATCH (0.0.x)**: Clarifications, wording, typos

### Sync Impact Report Format

```markdown
<!--
SYNC IMPACT REPORT
==================
Version Change: x.y.z → x.y.z
Modified Principles: [list]
Added Sections: [list]
Removed Sections: [list]
Clarifications: [list]
Templates Status:
  ✅/❌ .copilot/templates/plan-template.md
  ✅/❌ .copilot/templates/spec-template.md
  ✅/❌ .copilot/templates/tasks-template.md
Follow-up TODOs: [list]
Alignment: [status]
-->
```

### Output

Provide:
1. New version with rationale
2. Summary of changes
3. Template consistency status
4. Suggested commit message

## Example

**User**: "Add rate limiting principle for contact form"

**Response**:
- Version: 1.1.0 (MINOR - new guidance added)
- Modified: Technology Stack Constraints
- Templates: All aligned
- Commit: `docs: add rate limiting guidance to constitution v1.1.0`

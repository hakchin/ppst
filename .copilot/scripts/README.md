# Helper Scripts

Utility scripts for working with the spec-kit.

## Bash Scripts

### new-spec.sh

Creates a new specification directory with all necessary files.

**Usage**:
```bash
.copilot/scripts/bash/new-spec.sh
```

This script will:
1. Determine the next spec number (e.g., 001, 002, 003)
2. Prompt for a short name (kebab-case)
3. Create the spec directory: `specs/{number}-{name}/`
4. Copy templates for spec.md, plan.md, tasks.md
5. Create additional files: data-model.md, research.md, quickstart.md
6. Create subdirectories: checklists/, contracts/

**Example**:
```bash
$ .copilot/scripts/bash/new-spec.sh
Enter spec short name (kebab-case, e.g., 'homepage-contact'):
student-registration

Created spec: specs/002-student-registration
Next steps:
1. Edit specs/002-student-registration/spec.md with feature specification
2. Edit specs/002-student-registration/plan.md with implementation plan
3. Edit specs/002-student-registration/tasks.md with task breakdown
```

## Future Scripts

Additional scripts can be added here for:
- Validating constitution compliance
- Generating reports
- Automating checks
- Git workflow helpers

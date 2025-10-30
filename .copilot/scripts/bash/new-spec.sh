#!/bin/bash
# Helper script to create a new specification

set -e

# Get the next spec number
LAST_SPEC=$(ls -d specs/[0-9][0-9][0-9]-* 2>/dev/null | tail -1 | sed 's/specs\/\([0-9]*\)-.*/\1/')
if [ -z "$LAST_SPEC" ]; then
    NEXT_NUM="001"
else
    NEXT_NUM=$(printf "%03d" $((10#$LAST_SPEC + 1)))
fi

# Prompt for spec name
echo "Enter spec short name (kebab-case, e.g., 'homepage-contact'):"
read SPEC_NAME

# Create spec directory
SPEC_DIR="specs/${NEXT_NUM}-${SPEC_NAME}"
mkdir -p "$SPEC_DIR/checklists" "$SPEC_DIR/contracts"

# Copy templates
cp .copilot/templates/spec-template.md "$SPEC_DIR/spec.md"
cp .copilot/templates/plan-template.md "$SPEC_DIR/plan.md"
cp .copilot/templates/tasks-template.md "$SPEC_DIR/tasks.md"

# Create additional files
touch "$SPEC_DIR/data-model.md"
touch "$SPEC_DIR/research.md"
touch "$SPEC_DIR/quickstart.md"
touch "$SPEC_DIR/checklists/requirements.md"
touch "$SPEC_DIR/contracts/http-endpoints.md"

echo "Created spec: $SPEC_DIR"
echo "Next steps:"
echo "1. Edit $SPEC_DIR/spec.md with feature specification"
echo "2. Edit $SPEC_DIR/plan.md with implementation plan"
echo "3. Edit $SPEC_DIR/tasks.md with task breakdown"

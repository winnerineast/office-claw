#!/bin/bash
# Generated-by: [20260322-03-quality-gate-infra]

TASK_ID="20260322-03-quality-gate-infra"
REPORT_FILE="FAIL_REPORT.md"
RAW_LOG="test_output.log"

echo "🚀 Starting Quality Gate: OfficeClaw v1.3"

# 1. Run Tests without color
export NO_COLOR=1
cd core && npm test -- --no-color > ../$RAW_LOG 2>&1
TEST_EXIT_CODE=$?
cd ..

if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo "✅ Tests Passed."
    rm -f $RAW_LOG
    exit 0
else
    echo "❌ Tests Failed! Generating Report..."
    
    # Strip ANSI escape codes using sed (handles most common cases)
    # This ensures the Markdown file is readable on GitHub
    sed -i '' $'s/\e[[][^A-Za-z]*[A-Za-z]//g' $RAW_LOG 2>/dev/null || sed -i 's/\x1b\[[0-9;]*m//g' $RAW_LOG

    # Generate FAIL_REPORT.md
    echo "# ❌ [FAIL] Quality Gate Breach" > $REPORT_FILE
    echo "- **Task ID**: $TASK_ID" >> $REPORT_FILE
    echo "- **Timestamp**: $(date)" >> $REPORT_FILE
    echo "- **Status**: HANGING (Waiting for Human Decisions)" >> $REPORT_FILE
    echo "## Test Output" >> $REPORT_FILE
    echo '```text' >> $REPORT_FILE
    cat $RAW_LOG >> $REPORT_FILE
    echo '```' >> $REPORT_FILE
    
    echo "📄 Local report generated: $REPORT_FILE"
    
    # Try to create GitHub Issue if 'gh' is available
    if command -v gh &> /dev/null; then
        echo "🔗 Managing GitHub Labels..."
        
        ensure_label() {
            local label=$1
            local color=$2
            if ! gh label list --json name -q ".[].name" | grep -qx "$label"; then
                echo "Creating missing label: $label"
                gh label create "$label" --color "$color"
            fi
        }

        ensure_label "bug" "d73a4a"
        ensure_label "quality-gate" "0075ca"

        echo "🔗 Attempting to create GitHub Issue..."
        gh issue create --title "[FAIL] [$TASK_ID] Quality Gate Breach" --body-file $REPORT_FILE --label "bug,quality-gate"
    else
        echo "⚠️  GitHub CLI (gh) not found. Please create the issue manually."
    fi
    
    echo "🛑 Build Blocked. Please review $REPORT_FILE and fix the prompt."
    exit 1
fi

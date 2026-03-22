#!/bin/bash
# Generated-by: [20260322-03-quality-gate-infra]

# OfficeClaw Resource Budget Enforcer
LOC_LIMIT=15000
CORE_DIR="./core/src"

if [ ! -d "$CORE_DIR" ]; then
    echo "[BUDGET] Core source directory not found. Skipping LoC check."
    exit 0
fi

# Count both .ts and .js files, handle empty results
CURRENT_LOC=$(find "$CORE_DIR" -name "*.ts" -o -name "*.js" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}')

# Default to 0 if no files found
if [ -z "$CURRENT_LOC" ] || ! [[ "$CURRENT_LOC" =~ ^[0-9]+$ ]]; then
    CURRENT_LOC=0
fi

echo "📊 Current Core LoC: $CURRENT_LOC / $LOC_LIMIT"

if [ "$CURRENT_LOC" -gt "$LOC_LIMIT" ]; then
    echo "❌ [BUDGET BREACH] Codebase exceeds the 15k lines limit!"
    exit 1
fi

echo "✅ [BUDGET OK] Within limits."
exit 0

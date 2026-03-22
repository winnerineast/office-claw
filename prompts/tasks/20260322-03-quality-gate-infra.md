# 📋 Task: Quality Gate Infrastructure Setup (M1-Foundation)

- **ID**: `20260322-03-quality-gate-infra`
- **Target**: `core/`, `scripts/`, `github-issues`
- **Status**: In Progress

## 🎯 Objective
Establish the **Hexagonal Quality Gate** and the **HEG (Human-Enabled Governance)** loop. Ensure the system is robust across multiple runtimes (Shell, Node, Rust) and supports automated GitHub Issue management.

## 🛠️ Prompt
Implement the Stage 1 Quality Gate infrastructure for OfficeClaw with the following specifications:
1. **Core Setup**: 
   - Initialize `core/` as a Node.js ESM project with `vitest`.
2. **Comment Syntax Integrity**:
   - **MANDATORY**: Detect file types. Use `//` or `/*` for TypeScript/Rust and `#` for Shell scripts. Never use `/*` in a Bash script.
3. **Hexagonal Defense**:
   - `scripts/check-budget.sh`: LoC counter for `core/` (<15k).
4. **Automated Reporting & Label Management**:
   - `scripts/test-and-report.sh`:
     - Run `vitest` with `--no-color` or strip ANSI codes.
     - On failure: Generate `FAIL_REPORT.md`.
     - **Clean Logs**: Ensure the log is plain text without ANSI escape sequences to avoid garbled text in GitHub.
     - **Label Logic**: Check if GitHub labels `bug` and `quality-gate` exist using `gh label list`.
     - **Auto-Provision**: If a label is missing, create it using `gh label create`.
     - **Issue Creation**: Create the issue with the verified labels.
5. **Validation Test**:
   - `core/tests/quality-gate.test.ts` with a deliberate `[PROBE]` failure.

## 📂 Artifacts Generated
- `core/package.json`
- `core/vitest.config.ts`
- `core/tests/quality-gate.test.ts`
- `scripts/test-and-report.sh`
- `scripts/check-budget.sh`

## 🐛 Fix Log
- **v1.0**: Failed due to Shell comment syntax error and missing GitHub labels.
- **v1.1**: Fixed comment syntax to '#' for Bash. Added automated label check and creation logic for GitHub Issues.

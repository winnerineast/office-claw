# 🏗️ OfficeClaw Design Specification (v1.3)

/* Generated-by: [20260322-03-arch-finalization] */

This document defines the **Sequential Development Roadmap (v1.3)** and the **Closed-Loop Failure Tracking** mandate.

---

## 🗺️ Sequential Development Roadmap (v1.3)
OfficeClaw follows a strict incremental development cycle to ensure it is always **Compilable, Runnable, and Testable.**

### Stage 1: Quality Gate (The Foundation)
- **Goal**: Setup the automated test environment and Issue tracking infrastructure.
- **Artifacts**: `core/tests/`, `scripts/test-and-report.sh`.
- **Validation**: Successful GitHub Issue creation upon test failure.

### Stage 2: Onion Core (The Heart)
- **Goal**: Implement ACP WebSocket Gateway and the 6-layer Middleware Pipeline.
- **Artifacts**: `core/src/index.ts`, `core/src/pipeline/`.
- **Validation**: Message flow through all hooks with Mock responses.

### Stage 3: Security Bridge (The Shield)
- **Goal**: Rust NAPI-RS initialization and WASM Runtime execution.
- **Artifacts**: `security/src/lib.rs`, `security/wit/`.
- **Validation**: Verified FFI call from Node to Rust executing WASM byte-code.

### Stage 4: Isolation Layer (The Boundary)
- **Goal**: Multi-tenant containerization via Docker/OrbStack.
- **Artifacts**: `isolation/`, `scripts/docker-manager.sh`.
- **Validation**: Per-user container spawning with isolated persistent volumes.

### Stage 5: Privacy Router (NemoClaw)
- **Goal**: Real-time PII scrubbing (Regex/NLP).
- **Artifacts**: `routing/src/scrubber.ts`.
- **Validation**: Redacted sensitive data in `beforeLLM` pipeline stage.

### Stage 6: Fleet Governance (The Control Plane)
- **Goal**: Pull-based mTLS heartbeat and centralized billing/audit.
- **Artifacts**: `fleet/`, `dashboard/`.
- **Validation**: Hot-reloading of policies pushed from the Fleet server.

### Stage 7: Ecosystem Absorption (The Expansion)
- **Goal**: Seamlessly ingest OpenClaw's ClawHub skills and 20+ messaging channels.
- **Artifacts**: `core/src/adapters/clawhub/`.
- **Validation**: Execution of legacy OpenClaw plugins within the OfficeClaw sandbox.

---

## 🛠️ Operational Mandates
- **Prompt-First**: All code born from prompts in `prompts/tasks/`.
- **Atomic Commits**: Prompt and code committed together.
- **Failure-Driven**: Test failures MUST trigger a GitHub Issue; fixes MUST reference the Issue ID.

---
**Author**: OfficeClaw Architect (Gemini CLI)
**Last Updated**: 2026-03-22

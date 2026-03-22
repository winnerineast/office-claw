# 🏗️ OfficeClaw Design Specification (v1.2)

/* Generated-by: [20260322-01-project-init] */

This document integrates insights from the **2026 OpenClaw Ecosystem Research Report** and enterprise management requirements. It serves as the definitive guide for building a lightweight, secure, and enterprise-ready agent infrastructure.

---

## 1. Vision & Core Philosophy
OfficeClaw is **not** a fork that diverges; it is a **hardened downstream distribution**.
- **95% Smaller Footprint**: Focused core (~15k lines).
- **Hardware-Level Security**: Rust/WASM-based physical isolation.
- **Privacy First**: Real-time PII scrubbing.
- **Fleet Governance**: Centralized control for enterprise-wide deployment and compliance.
- **Prompt-First Engineering**: Every line of code is born from a version-tracked prompt.

---

## 2. Architectural Inheritance (OpenClaw Compatibility)
- **ACP Standard**: Mandatory implementation of Agent Client Protocol.
- **Plugin SDK**: Supports `@mariozechner/pi-agent-core` interfaces.
- **Configuration**: JSON5 `openclaw.json` with an extended `office` namespace.

---

## 3. The Bridge Architecture Layers
### 3.1 Logic Layer (The Core)
- Minimal reasoning loop and task dispatching.
### 3.2 Security Layer (IronClaw - Rust/WASM)
- WASM-based sandboxing with CPU/Memory fuel limits.
### 3.3 Isolation Layer (NanoClaw - Containers)
- Per-user MicroVM/Container isolation.
### 3.4 Routing Layer (NemoClaw - Privacy Router)
- Real-time PII scrubbing and intelligent model dispatching.
### 3.5 Governance Layer (Fleet Management)
- Centralized deployment, billing control, and encrypted audit logs.

---

## 4. Coding Mandate: Prompt-First Strategy
To maintain absolute architectural integrity, OfficeClaw follows a strict **Prompt-First** workflow:
1. **Task-Based Prompts**: Prompts are stored in `prompts/tasks/`.
2. **Atomic Commits**: Code and prompts must be committed together.
3. **Prompt-Centric Debugging**: Fix bugs by enhancing the generating prompt, not by patching code manually.

---

## 5. Security & Compliance Mandates
- **Zero-Trust Auth**: Enforced Token Authentication and DM Pairing.
- **Auditability**: Cryptographically signed audit logs for every agent action.

---

## 6. Roadmap to V1
- [ ] **Stage 1**: ACP-compatible Gateway with basic "Hello World" Skill in WASM.
- [ ] **Stage 2**: Integration of Feishu/Telegram channels and Fleet heartbeat.
- [ ] **Stage 3**: Local PII scrubbing with MLX-based small models.
- [ ] **Stage 4**: Multi-node cluster deployment with Centralized Billing.

---
**Author**: OfficeClaw Architect (Gemini CLI)
**Last Updated**: 2026-03-22

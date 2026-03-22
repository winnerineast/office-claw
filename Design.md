# 🏗️ OfficeClaw Design Specification (v1.2)

This document integrates insights from the **2026 OpenClaw Ecosystem Research Report** and enterprise management requirements. It serves as the definitive guide for building a lightweight, secure, and enterprise-ready agent infrastructure.

---

## 1. Vision & Core Philosophy
OfficeClaw is **not** a fork that diverges; it is a **hardened downstream distribution**.
- **95% Smaller Footprint**: By stripping personal-use experimental features and focusing on core OA (Office Automation) logic.
- **Hardware-Level Security**: Moving from JS-based "logic checks" to Rust/WASM-based "physical isolation."
- **Privacy First**: Integrating real-time PII scrubbing into the communication stream.
- **Fleet Governance**: Centralized control for enterprise-wide deployment and compliance.

---

## 2. Architectural Inheritance (OpenClaw Compatibility)
To ensure seamless integration with the existing OpenClaw ecosystem (Clawhub, Skills, Channels), OfficeClaw adheres to these standards:

### 2.1 Agent Client Protocol (ACP)
- **Status**: Mandatory.
- **Role**: OfficeClaw's `core/` will implement the ACP gateway, allowing any OpenClaw-compatible client (Mobile, Desktop, CLI) to connect without modification.

### 2.2 Plugin SDK & Runtime
- **Strategy**: Re-implement or wrapper the `plugin-sdk/*` modules.
- **Mechanism**: Skills expecting `AgentTool` (via `@mariozechner/pi-agent-core`) will find an identical interface in OfficeClaw, but the underlying execution will be proxied to the **IronClaw WASM Bridge**.

### 2.3 Configuration Standard
- **Format**: `openclaw.json` (JSON5).
- **Extension**: OfficeClaw respects all standard OpenClaw keys but adds an `office` namespace for enterprise-specific settings (e.g., `office.pii_redaction: true`, `office.fleet_id: "finance-dept"`).

---

## 3. The Bridge Architecture Layers

### 3.1 Logic Layer (The Core)
- **Focus**: Minimal reasoning loop and task dispatching based on **OpenClaw/Nanoclaw**.
- **Constraint**: No direct access to host resources. All must go through the Security Layer.

### 3.2 Security Layer (IronClaw - Rust/WASM)
- **Role**: The "Gatekeeper."
- **Execution**: Every Skill runs in a **WASM Component** environment.
- **Limits**: Hard-coded CPU fuel and memory quotas (inspired by **ZeroClaw**).

### 3.3 Isolation Layer (NanoClaw - Containers)
- **Role**: Multi-tenant privacy.
- **Implementation**: Each user session runs in a dedicated **MicroVM/Container**.

### 3.4 Routing Layer (NemoClaw - Privacy Router)
- **Role**: Intelligent, privacy-aware task dispatching.
- **PII Scrubbing**: Real-index and NLP-based redaction before cloud routing.

### 3.5 Governance Layer (Fleet Management)
- **Role**: The "Enterprise Control Plane."
- **Remote Orchestration**: Centralized deployment, OTA updates, and health monitoring of all OfficeClaw nodes.
- **Billing Control**: Unified LLM API quota management and department-level budget tracking.
- **Compliance**: Cryptographically signed audit logs for every agent action.

---

## 4. Implementation Strategy: "Core Peeling"
Instead of building from scratch, we "peel" the heavy layers of OpenClaw:
1.  **Extract**: Identify the minimal set of dependencies.
2.  **Redirect**: Swap `node:fs` calls with **IronClaw Bridge** calls.
3.  **Harden**: Replace the JS skill loader with the Rust-WASM loader.

---

## 5. Security & Compliance Mandates
- **Zero-Trust Auth**: Mandatory Token Authentication (Post-ClawJacked fix).
- **Auditability**: Every tool execution must generate a signed audit event.
- **Data Sovereignty**: Local-first processing; PII never leaves the cluster.

---

## 6. Roadmap to V1
- [ ] **Stage 1**: ACP-compatible Gateway with basic "Hello World" Skill in WASM.
- [ ] **Stage 2**: Integration of Feishu/Telegram channels and Fleet heartbeat.
- [ ] **Stage 3**: Local PII scrubbing with MLX-based small models.
- [ ] **Stage 4**: Multi-node cluster deployment with Centralized Billing.

---
**Author**: OfficeClaw Architect (Gemini CLI)
**Last Updated**: 2026-03-22

# 🏗️ OfficeClaw Design Specification (v1.0)

This document outlines the architectural blueprints, design principles, and compatibility strategies for **OfficeClaw**, ensuring it remains a lightweight, secure, and enterprise-ready enhancement of the OpenClaw ecosystem.

---

## 1. Vision & Core Philosophy
OfficeClaw is **not** a fork that diverges; it is a **hardened downstream distribution**.
- **95% Smaller Footprint**: By stripping personal-use experimental features and focusing on core OA (Office Automation) logic.
- **Hardware-Level Security**: Moving from JS-based "logic checks" to Rust/WASM-based "physical isolation."
- **Privacy First**: Integrating real-time PII scrubbing into the communication stream.

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
- **Extension**: OfficeClaw respects all standard OpenClaw keys but adds an `office` namespace for enterprise-specific settings (e.g., `office.pii_redaction: true`, `office.isolation_mode: "container"`).

---

## 3. The Bridge Architecture Layers

### 3.1 Logic Layer (The Core)
- **Focus**: Minimal reasoning loop and task dispatching.
- **Constraint**: No direct access to host resources (FS, Network, Process). All must go through the Security Layer.

### 3.2 Security Layer (IronClaw - Rust/WASM)
- **Role**: The "Gatekeeper."
- **Execution**: Every Skill is compiled to (or interpreted as) a WASM Component. 
- **Limits**: Hard-coded CPU fuel and memory quotas (default 10MB) enforced by the Rust runtime.

### 3.3 Isolation Layer (NanoClaw - Containers)
- **Role**: Multi-tenant privacy.
- **Implementation**: Each `AgentScope` is bound to a dedicated OS-level container (via OrbStack/Docker). 
- **Persistence**: Per-user volume mounts; zero cross-contamination.

### 3.4 Routing Layer (NemoClaw - Privacy Router)
- **Role**: Intelligent, privacy-aware task dispatching.
- **PII Scrubbing**: Real-time regex and NLP-based redaction of Emails, SSNs, and Financial data before cloud routing.

---

## 4. Implementation Strategy: "Core Peeling"
Instead of building from scratch, we "peel" the heavy layers of OpenClaw:
1.  **Extract**: Identify the minimal set of dependencies in `package.json`.
2.  **Redirect**: Swap `node:fs` and `node:child_process` calls with **IronClaw Bridge** calls.
3.  **Harden**: Replace the JS-based skill loader with the Rust-based WASM loader.

---

## 5. Security Mandates
- **No Direct Shell**: `exec` and `spawn` are strictly forbidden outside the WASM sandbox.
- **Cryptographic Provenance**: Only signed Skills from trusted hubs (or local verified builds) can be loaded.
- **Ephemeral Runtimes**: Agent containers are recycled or deeply scrubbed after session termination.

---

## 6. Roadmap to V1
- [ ] **Stage 1**: ACP-compatible Gateway with basic "Hello World" Skill in WASM.
- [ ] **Stage 2**: Integration of OpenClaw's `Feishu` and `Telegram` channels.
- [ ] **Stage 3**: Local PII scrubbing with MLX-based small models.
- [ ] **Stage 4**: Multi-node cluster deployment via `scripts/deploy.sh`.

---
**Author**: OfficeClaw Architect (Gemini CLI)
**Last Updated**: 2026-03-22

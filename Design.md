# 🏗️ OfficeClaw Design Specification (v1.2 - Deep Dive)

/* Generated-by: [20260322-02-arch-deep-dive] */

This document defines the **Core Primitives**, **Integration Hooks**, and **Fleet Topology** for OfficeClaw. It prioritizes extreme minimalism, hardware-enforced security, and centralized enterprise governance.

---

## 1. Architectural Synthesis (Variant Lessons)
OfficeClaw's design is a synthesis of the most successful OpenClaw variants:
- **ZeroClaw/PicoClaw Minimalism**: Use `node:http` and pure JS without heavy frameworks (Express/Nest) to maintain a <10MB idle memory footprint.
- **NanoClaw Isolation**: The `core` assumes a "Hostile Environment" and relies on the **Isolation Layer** for filesystem boundaries.
- **IronClaw FFI-First**: The `core` interacts with `security` (Rust) via high-performance FFI (NAPI-RS), not via shell spawning.

---

## 2. Core Primitives & Integration (The Onion Model)
All incoming messages through ACP follow a strictly defined **Onion Middleware Pipeline** to ensure security and privacy at every step.

### 2.1 The Middleware Pipeline
1.  **Ingress**: Receive raw ACP payload.
2.  **Hook: `onMessageReceived`**: Fleet-level filtering for global policy enforcement.
3.  **Hook: `beforeLLM` (NemoClaw)**: Scrub PII (Emails, SSNs, Private Names) before cloud routing.
4.  **Reasoning**: Core agent logic determines the next step (Response or Skill execution).
5.  **Hook: `beforeSkill` (IronClaw)**: Verify the WASM component's signature and resource quota.
6.  **Egress**: Return the encrypted, audited response to the channel.

### 2.2 Interface Contracts (Dependency Injection)
The `core` defines strict interfaces for external modules to prevent tight coupling:
- **`ISecurityBridge`**: `async execute(wasmId, payload) -> Result`
- **`IPrivacyRouter`**: `async redact(text) -> redactedText`
- **`IIsolationProvider`**: `async spawnUserAgent(userId) -> containerId`

---

## 3. Fleet Connectivity & Cluster Topology
Enterprise deployment requires a **Pull-based Control Plane** to navigate firewalls and NATs.

### 3.1 Pull-based Heartbeat
- **Core-Initiated**: Every OfficeClaw node initiates a persistent connection to the central Fleet server upon boot.
- **mTLS Auth**: All Core-to-Fleet communication is encrypted via mutual TLS with short-lived certificates.
- **Hot-Reload Strategy**: Policies (e.g., "Disable Skill: Browser") and Billing Quotas (e.g., "$10 daily limit") are pushed from Fleet and hot-reloaded by Core without restart.

### 3.2 Global Audit Bus
- **Event-Driven**: All critical events (Skill calls, Model access, Auth failures) are emitted to a non-blocking `EventBus`.
- **Signed Logs**: The Fleet module batches these events, cryptographically signs them locally, and pushes them to the central server for compliance (SOC2/GDPR).

---

## 4. Implementation Constraints
- **Zero-Config Deployment**: Initial setup must only require a single `FLEET_TOKEN`.
- **Stateless Orchestrator**: The Node.js core remains stateless; all persistence is delegated to the **Isolation Layer** (MicroVM storage).
- **Graceful Degradation**: If Fleet is unreachable, Core falls back to the "Last Known Good Policy" but restricts model access to local MLX only.

---
**Author**: OfficeClaw Architect (Gemini CLI)
**Last Updated**: 2026-03-22

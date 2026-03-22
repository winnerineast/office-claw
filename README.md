# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

/* Generated-by: [20260322-02-arch-deep-dive] */

[中文版 (Chinese)](README_CN.md) | **English**

**OfficeClaw** is a production-grade AI agent framework specifically designed for modern office automation (OA) environments. It leverages the powerful orchestration of **OpenClaw** while introducing a "Bridge Architecture" to meet the rigorous demands of enterprise deployment: **Security, Privacy, and Resource Efficiency.**

---

## 📜 Credits & References
OfficeClaw is built upon the foundational work and architectural concepts of the following pioneering projects:

- **[OpenClaw](https://github.com/openclaw/openclaw)**: The core personal AI assistant framework that provides our multi-channel orchestration logic.
- **[IronClaw](https://github.com/nearai/ironclaw)**: Inspirational Rust-based implementation specializing in high-performance and WASM sandboxing.
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**: The benchmark for lightweight, OS-level container isolation.
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**: The standard for enterprise-grade privacy routing and policy governance.
- **[ZeroClaw](https://github.com/winnerineast/openclaw)**: A benchmark for ultra-fast startup (<10ms) and minimal static binary footprint.
- **[PicoClaw](https://github.com/winnerineast/openclaw)**: The gold standard for low-memory efficiency (~1MB footprint).

---

## 🛡️ Three Core Pillars

### 1. 🚀 Lightweight (Inspired by ZeroClaw/PicoClaw)
- **Small Footprint**: Optimized core with ~15k lines of code.
- **Efficient Runtime**: <10MB idle memory consumption.
- **Fast Cold Start**: Near-instant activation via WASM component pre-loading.

### 2. 🔐 Enterprise Security (Inspired by IronClaw/NanoClaw)
- **WASM Sandbox**: Cryptographically isolated execution for all Skills via high-performance FFI (NAPI-RS).
- **Container Isolation**: OS-level isolation for each user agent to prevent cross-role data leaks.
- **Zero-Trust Auth**: Enforced Token Authentication and **mTLS** encryption to prevent exploits like "ClawJacked."

### 3. 📊 Fleet Governance (Inspired by NemoClaw)
- **Pull-based Control Plane**: Remote deployment, updates, and health monitoring of all office nodes.
- **Billing & Quota Control**: Unified management of LLM costs and department-level budgets.
- **Compliance Auditing**: Signed audit logs for every agent action to satisfy SOC2/GDPR.

---

## 🏗️ Bridge Architecture v1.2 (Deep Dive)
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:
- **Onion Middleware Pipeline**: A 6-step rigorous processing flow:
  1. **Ingress**: Receive raw ACP payload.
  2. **Fleet Hook**: Policy-based filtering.
  3. **PII Scrub (NemoClaw)**: Redaction of sensitive data.
  4. **Reasoning**: Core agent logic.
  5. **WASM Verify (IronClaw)**: Security validation.
  6. **Egress**: Final audited response.
- **Security Layer**: Integrated **IronClaw/ZeroClaw** concepts with high-performance FFI (NAPI-RS).
- **Isolation Layer**: Integrated **NanoClaw** microVM/container concepts.
- **Governance Layer**: Integrated **NemoClaw** and custom **Pull-based Fleet Management**.

---

## 🛠️ Coding Mandate: Prompt-First
Every line of code in OfficeClaw is born from a version-tracked prompt.
- **Task-Based Prompts**: Stored in `prompts/tasks/`.
- **Atomic Commits**: Code and prompts are committed together.
- **Fix Protocol**: Enhance the prompt first, then re-generate the code.

---

## 📂 Project Structure
```text
office-claw/
├── security/           # Rust-based WASM Security Bridge
├── core/               # Enterprise Orchestrator
├── routing/            # Privacy & Model Router
├── isolation/          # Container Management Layer
├── fleet/              # Fleet Management & Control Plane
├── dashboard/          # Management UI & Audit Logs
├── prompts/tasks/      # Task-based generating prompts
└── scripts/            # Deployment & Maintenance Tools
```

---
**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.2 (Symmetrical)`

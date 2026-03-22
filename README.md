# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

/* Generated-by: [20260322-02-arch-deep-dive] */

[中文版 (Chinese)](README_CN.md) | **English**

**OfficeClaw** is a production-grade AI agent framework specifically designed for modern office automation (OA) environments. It leverages the powerful orchestration of **OpenClaw** while introducing a "Bridge Architecture" to meet the rigorous demands of enterprise deployment: **Security, Privacy, and Resource Efficiency.**

---

## 🛡️ Three Core Pillars
1. **🚀 Lightweight (Inspired by ZeroClaw/PicoClaw)**: Optimized core with ~15k lines of code and <10MB idle memory.
2. **🔐 Enterprise Security (Inspired by IronClaw/NanoClaw)**: WASM-based sandboxing, OS-level container isolation, and **mTLS** authentication.
3. **📊 Fleet Governance (Inspired by NemoClaw)**: **Pull-based** centralized management for deployment, billing control, and compliance auditing.

---

## 🏗️ Bridge Architecture v1.2 (Deep Dive)
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:
- **Onion Middleware Pipeline**: A 6-step processing flow (Ingress -> Fleet Hook -> PII Scrub -> Reasoning -> WASM Verify -> Egress).
- **Security Layer**: Integrated **IronClaw/ZeroClaw** concepts with high-performance FFI (NAPI-RS).
- **Isolation Layer**: Integrated **NanoClaw** microVM/container concepts for per-user filesystem privacy.
- **Governance Layer**: Integrated **NemoClaw** and custom **Pull-based Fleet Management** for remote orchestration and signed audit logs.

---

## 📂 Project Structure
```text
office-claw/
├── security/           # Rust-based WASM Security Bridge (IronClaw/ZeroClaw concepts)
├── core/               # Enterprise Orchestrator (based on OpenClaw logic)
├── routing/            # Privacy & Model Router (NemoClaw)
├── isolation/          # Container Management Layer (NanoClaw)
├── fleet/              # Fleet Management & Control Plane
├── dashboard/          # Management UI & Audit Logs
├── prompts/tasks/      # Task-based generating prompts (Prompt-First mandate)
└── scripts/            # Deployment & Maintenance Tools
```

---
**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.2 (Refined)`

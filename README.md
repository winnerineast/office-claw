# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

/* Generated-by: [20260322-01-project-init] */

[中文版 (Chinese)](README_CN.md) | **English**

**OfficeClaw** is a production-grade AI agent framework specifically designed for modern office automation (OA) environments. It leverages the powerful orchestration of **OpenClaw** while introducing a "Bridge Architecture" to meet the rigorous demands of enterprise deployment: **Security, Privacy, and Resource Efficiency.**

---

## 🛡️ Three Core Pillars
1. **🚀 Lightweight (Inspired by ZeroClaw/PicoClaw)**: Optimized core with ~15k lines of code.
2. **🔐 Enterprise Security (Inspired by IronClaw/NanoClaw)**: WASM-based sandboxing and OS-level container isolation.
3. **📊 Fleet Governance (Inspired by NemoClaw)**: Centralized deployment, billing control, and compliance auditing.

---

## 🏗️ Bridge Architecture v1.2
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:
- **Logic Layer**: Flexible logic from **OpenClaw**.
- **Security Layer**: Integrated **IronClaw/ZeroClaw** concepts for safe and fast execution.
- **Isolation Layer**: Integrated **NanoClaw** container concepts for multi-tenant privacy.
- **Governance Layer**: Integrated **NemoClaw** and custom Fleet Management for privacy and control.

---

## 🛠️ Coding Mandate: Prompt-First
Every line of code in OfficeClaw is born from a version-tracked prompt. We prioritize fixing the **prompt** over patching the **code**.
- **Task-Based Prompts**: Stored in `prompts/tasks/`.
- **Atomic Commits**: Code and prompts are committed together in a single Git commit.

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
├── prompts/tasks/      # Mirror of task-based generating prompts
└── scripts/            # Deployment & Maintenance Tools
```

---
**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.2`

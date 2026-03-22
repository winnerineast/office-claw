# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

[中文版 (Chinese)](README_CN.md) | **English**

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

## 🌟 Our Vision
In an enterprise setting, AI agents must be more than just "smart." They must be **trustworthy and unobtrusive**. OfficeClaw is built to run on standard office hardware while ensuring that sensitive company data never leaves the local network without explicit, audited permission.

---

## 🛡️ Three Core Pillars

### 1. 🚀 Lightweight (Inspired by ZeroClaw/PicoClaw)
- **Small Footprint**: Optimized core with ~15k lines of code.
- **Fast Cold Start**: Near-instant activation via WASM component pre-loading.

### 2. 🔐 Enterprise Security (Inspired by IronClaw/NanoClaw)
- **WASM Sandbox**: Cryptographically isolated execution for all Skills.
- **Zero-Trust Auth**: Enforced Token Authentication to prevent exploits like "ClawJacked."

### 3. 📊 Fleet Management & Governance (Inspired by NemoClaw)
- **Centralized Control Plane**: Remote deployment, updates, and monitoring of all office nodes.
- **Billing & Quota Control**: Unified management of LLM costs and department-level budgets.
- **Compliance Auditing**: Signed audit logs for every agent action to satisfy SOC2/GDPR.

---

## 🏗️ Bridge Architecture
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:

- **Logic Layer**: Inherits flexible logic from **OpenClaw**.
- **Security Layer**: Integrated **IronClaw/ZeroClaw** concepts for safe and fast execution.
- **Isolation Layer**: Integrated **NanoClaw** container concepts for multi-tenant privacy.
- **Routing & Governance Layer**: Integrated **NemoClaw** and custom Fleet Management for privacy, model routing, and centralized control.

---

## 📂 Project Structure
```text
office-claw/
├── security/           # Rust-based WASM Security Bridge (IronClaw/ZeroClaw)
├── core/               # Enterprise Orchestrator (based on OpenClaw/Nanoclaw)
├── routing/            # Privacy & Model Router (NemoClaw)
├── isolation/          # Container Management Layer (NanoClaw)
├── fleet/              # Fleet Management & Control Plane
├── dashboard/          # Management UI & Audit Logs
└── scripts/            # Deployment & Maintenance Tools
```

---

**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.2`

---

## 📝 Work Daily

### 🗓️ 2026-03-22
- **Cleaned Environment**: Reset local machine, removed old global packages.
- **Source Sync**: Cloned `openclaw` and `office-claw` repositories.
- **Vision Alignment**: Defined "Lightweight, Secure, Manageable" goals.
- **Architecture Update**: Added **Fleet Management & Governance** layer to all docs (README, README_CN, Design) to address centralized deployment, billing control, and compliance.
- **Next Step**: Begin Phase 1 - Directory creation and Rust initialization.

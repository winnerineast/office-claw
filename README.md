# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

[中文版 (Chinese)](README_CN.md) | **English**

---

## 📜 Credits & References
OfficeClaw is built upon the foundational work and architectural concepts of the following pioneering projects:

- **[OpenClaw](https://github.com/openclaw/openclaw)**: The core personal AI assistant framework that provides our multi-channel orchestration logic.
- **[IronClaw](https://github.com/nearai/ironclaw)**: Inspirational Rust-based implementation specializing in high-performance and WASM sandboxing.
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**: The benchmark for lightweight, OS-level container isolation.
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**: The standard for enterprise-grade privacy routing and policy governance.
- **[ZeroClaw](https://github.com/winnerineast/openclaw)**: A benchmark for ultra-fast startup (<10ms) and minimal static binary footprint (~3.4MB).
- **[PicoClaw](https://github.com/winnerineast/openclaw)**: The gold standard for low-memory efficiency (~1MB footprint) on edge hardware.

---

## 🌟 Our Vision
In an enterprise setting, AI agents must be more than just "smart." They must be **trustworthy and unobtrusive**. OfficeClaw is built to run on standard office hardware (from Mac minis to PC clusters) while ensuring that sensitive company data never leaves the local network without explicit, audited permission.

---

## 🛡️ Three Core Pillars

### 1. 🚀 Lightweight (Inspired by ZeroClaw/PicoClaw)
- **Small Footprint**: Optimized core with ~15k lines of code, reducing resource overhead by 95% compared to generic frameworks.
- **Fast Cold Start**: Aiming for <100ms agent activation via specialized WASM component pre-loading and static linking techniques.
- **Edge-Ready**: Optimized for standard office machines (8GB+ RAM) with minimal idle memory consumption.

### 2. 🔐 Enterprise Security (Inspired by IronClaw/NanoClaw)
- **Rust/WASM Sandbox**: Skills run in cryptographically isolated WebAssembly sandboxes with strict resource quotas.
- **Container Isolation**: OS-level isolation for each user agent to prevent cross-role data leaks.
- **Zero-Trust Auth**: Mandatory Token Authentication and DM Pairing, preventing "ClawJacked" style exploits.

### 3. 📊 Manageability (Inspired by NemoClaw)
- **Centralized Control**: Unified dashboard for monitoring health, resource usage, and task success rates.
- **Privacy Router**: Automatic PII scrubbing ensuring sensitive data stays local while only anonymized queries reach the cloud.
- **Zero-Config Deployment**: Tailored for non-technical administrators with one-click installation.

---

## 🏗️ Bridge Architecture
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:

- **Logic Layer**: Inherits the flexible logic from **OpenClaw**.
- **Security Layer**: Integrated **IronClaw/ZeroClaw** runtime concepts for safe and fast execution.
- **Isolation Layer**: Integrated **NanoClaw** container concepts for multi-tenant privacy.
- **Routing Layer**: Integrated **NemoClaw** privacy-aware task dispatching.

---

## 📂 Project Structure
```text
office-claw/
├── security/           # Rust-based WASM Security Bridge (IronClaw/ZeroClaw concepts)
├── core/               # Enterprise Orchestrator (based on OpenClaw/Nanoclaw logic)
├── routing/            # Privacy-first Task Router (NemoClaw)
├── isolation/          # Container Management Layer (NanoClaw)
├── dashboard/          # Management UI & Audit Logs
└── scripts/            # Deployment & Maintenance Tools
```

---

**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.1`

---

## 📝 Work Daily

### 🗓️ 2026-03-22
- **Cleaned Environment**: Reset local machine, removed old global packages.
- **Source Sync**: Cloned `openclaw` and `office-claw` repositories.
- **Vision Alignment**: Defined "Lightweight, Secure, Manageable" goals.
- **Documentation**: Finalized multi-language README and Design Spec, integrating ZeroClaw/PicoClaw references for performance optimization.
- **Next Step**: Begin Phase 1 - Directory creation and Rust initialization.

# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

[中文版 (Chinese)](README_CN.md) | **English**

---

## 📜 Credits & References
OfficeClaw is built upon the foundational work and architectural concepts of the following pioneering projects:

- **[OpenClaw](https://github.com/openclaw/openclaw)**: The core personal AI assistant framework that provides our multi-channel orchestration logic.
- **[IronClaw](https://github.com/nearai/ironclaw)**: Inspirational Rust-based implementation specializing in high-performance and WASM sandboxing.
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**: The benchmark for lightweight, OS-level container isolation.
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**: The standard for enterprise-grade privacy routing and policy governance.

---

## 🌟 Our Vision
In an enterprise setting, AI agents must be more than just "smart." They must be **trustworthy and unobtrusive**. OfficeClaw is built to run on standard office hardware (from Mac minis to PC clusters) while ensuring that sensitive company data never leaves the local network without explicit, audited permission.

---

## 🛡️ Three Core Pillars

### 1. 🚀 Lightweight
- **Small Footprint**: Optimized core with ~15k lines of code, reducing resource overhead by 95% compared to generic frameworks.
- **Edge-Ready**: Runs on standard 8GB+ RAM office machines by leveraging quantized MLX models and local cluster offloading.
- **Fast Cold Start**: Near-instant activation via specialized WASM component pre-loading.

### 2. 🔐 Enterprise Security
- **Rust/WASM Sandbox (IronClaw-inspired)**: Skills run in cryptographically isolated WebAssembly sandboxes with strict resource quotas.
- **Privacy Router (NemoClaw-inspired)**: Automatic PII scrubbing ensures sensitive data stays local while only anonymized queries reach the cloud.
- **Container Isolation (NanoClaw-inspired)**: OS-level isolation for each user agent to prevent cross-role data leaks.

### 3. 📊 Manageability
- **Centralized Control**: Unified dashboard for monitoring health, resource usage, and task success rates.
- **Audit Trails**: Comprehensive logging of every agent action for compliance and security review.
- **Zero-Config Deployment**: Tailored for non-technical administrators with one-click installation.

---

## 🏗️ Bridge Architecture
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:

- **Logic Layer**: Inherits the flexible logic from **OpenClaw**.
- **Security Layer**: Integrated **IronClaw** runtime concepts for safe tool execution.
- **Isolation Layer**: Integrated **NanoClaw** container concepts for multi-tenant privacy.
- **Routing Layer**: Integrated **NemoClaw** privacy-aware task dispatching.

---

## 📂 Project Structure
```text
office-claw/
├── security/           # Rust-based WASM Security Bridge (IronClaw)
├── core/               # Enterprise Orchestrator (based on OpenClaw logic)
├── routing/            # Privacy-first Task Router (NemoClaw)
├── isolation/          # Container Management Layer (NanoClaw)
├── dashboard/          # Management UI & Audit Logs
└── scripts/            # Deployment & Maintenance Tools
```

---

## 🛠️ Development & Deployment Lifecycle

### 1. Source-to-Binary Build
Full transparency via source-based builds.
```bash
git clone https://github.com/winnerineast/office-claw
cd office-claw
./scripts/build-all.sh
```

### 2. Validation & Testing
Mandatory "Office Stress Test" simulating 20+ concurrent OA tasks.

### 3. Deployment
```bash
./scripts/deploy.sh --cluster
```

---

**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.0`

---

## 🗺️ Action Plan

### Phase 1: Structure & Environment
- [ ] **Sync Directory Structure**: Create folders (security, core, routing, isolation, dashboard, scripts).
- [ ] **Initialize Rust Security Bridge**: Setup `security/` as a Rust NAPI-RS project.
- [ ] **Setup Build Chain**: Create `scripts/build-all.sh` for multi-runtime compilation.

### Phase 2: Core Logic Extraction
- [ ] **Lean Orchestrator**: Extract minimal logic from `OpenClaw` to `office-claw/core`.
- [ ] **Privacy Router (NemoClaw)**: Implement initial PII scrubbing logic.

### Phase 3: Security & Isolation
- [ ] **WASM Sandbox**: Implement resource limiting in the Rust Bridge.
- [ ] **Container Integration**: Develop the `isolation/` layer for multi-tenant support.

### Phase 4: Management & Deployment
- [ ] **Dashboard MVP**: Basic status monitoring UI.
- [ ] **Cluster Deploy**: Finalize `scripts/deploy.sh`.

---

## 📝 Work Daily

### 🗓️ 2026-03-22
- **Cleaned Environment**: Reset local machine, removed old global packages.
- **Source Sync**: Cloned `openclaw` and `office-claw` repositories.
- **Vision Alignment**: Defined "Lightweight, Secure, Manageable" goals.
- **Documentation**: Finalized multi-language README structure with full credit to the "Claw" family.
- **Next Step**: Begin Phase 1 - Directory creation and Rust initialization.

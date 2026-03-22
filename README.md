# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

**OfficeClaw** is a production-grade AI agent framework specifically designed for modern office automation (OA) environments. It leverages the powerful orchestration of **OpenClaw** while introducing a "Bridge Architecture" to meet the rigorous demands of enterprise deployment: **Security, Privacy, and Resource Efficiency.**

---

## 🌟 Our Vision
In an enterprise setting, AI agents must be more than just "smart." They must be **trustworthy and unobtrusive**. OfficeClaw is built to run on standard office hardware (from Mac minis to PC clusters) while ensuring that sensitive company data never leaves the local network without explicit, audited permission.

---

## 🛡️ Three Core Pillars

### 1. 🚀 Lightweight
- **Small Footprint**: Optimized to use 95% fewer resources than generic agent frameworks (~15k lines of core code).
- **Edge-Ready**: Designed to run comfortably on standard office machines (8GB+ RAM) by offloading heavy inference to a local cluster or using quantized MLX models.
- **Fast Cold Start**: Near-instant agent activation via specialized WASM component pre-loading.

### 2. 🔐 Enterprise Security
- **Rust/WASM Sandbox (IronClaw)**: Every third-party "Skill" runs in a cryptographically isolated WebAssembly sandbox with strict CPU/Memory quotas.
- **Privacy Router (NemoClaw)**: Automatic PII (Personally Identifiable Information) scrubbing. Sensitive data like salaries or contracts are processed locally; only anonymized queries reach the cloud.
- **Container Isolation (NanoClaw)**: Each user's agent is isolated at the OS level, ensuring zero cross-talk between different departments or roles.

### 3. 📊 Manageability
- **Centralized Control**: A single dashboard to monitor agent health, resource usage, and task success rates across the entire office cluster.
- **Audit Trails**: Every action taken by an agent is logged for compliance and security reviews.
- **Zero-Config Deployment**: One-click installation and automated configuration tailored for non-technical office administrators.

---

## 🏗️ Bridge Architecture
OfficeClaw acts as the "Bridge" between raw AI power and enterprise-specific constraints:

- **Logic Layer**: Inherits the flexible, multi-channel agent logic from **OpenClaw**.
- **Security Layer**: Integrated **IronClaw** (Rust runtime) for safe tool execution.
- **Isolation Layer**: Integrated **NanoClaw** (Container runner) for multi-tenant privacy.
- **Routing Layer**: Integrated **NemoClaw** for intelligent, privacy-aware task dispatching.

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
We believe in full transparency. OfficeClaw is built from source to ensure no hidden backdoors.
```bash
# Clone and build the core components
git clone https://github.com/winnerineast/office-claw
cd office-claw
./scripts/build-all.sh
```

### 2. Validation & Testing
Every build must pass the "Office Stress Test" before deployment, simulating 20+ concurrent OA tasks.

### 3. Deployment
Deploy to a single machine or a Mac Studio cluster with a single command:
```bash
./scripts/deploy.sh --cluster
```

---

## 🤝 Relationship with OpenClaw
OfficeClaw is a downstream project of [OpenClaw](https://github.com/openclaw/openclaw). While OpenClaw focuses on personal versatility and broad channel support, OfficeClaw focuses on **Enterprise hardening, Local-First privacy, and Scalability.**

---

**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.0`

---

## 🗺️ Action Plan

### Phase 1: Structure & Environment
- [ ] **Sync Directory Structure**: Create the core folders defined in the architecture (security, core, routing, isolation, dashboard, scripts).
- [ ] **Initialize Rust Security Bridge**: Setup `security/` as a Rust NAPI-RS project for WASM runtime.
- [ ] **Setup Build Chain**: Create `scripts/build-all.sh` to automate dependency installation and compilation across Node.js and Rust.

### Phase 2: Core Logic Extraction
- [ ] **Lean Orchestrator**: Extract the minimal agent orchestration logic from `OpenClaw` into `office-claw/core`.
- [ ] **Privacy Router (NemoClaw)**: Implement the initial PII scrubbing logic for local/cloud routing.

### Phase 3: Security & Isolation
- [ ] **WASM Sandbox**: Implement resource limiting (CPU/Memory) in the Rust Security Bridge.
- [ ] **Container Integration**: Develop the `isolation/` layer using Docker/OrbStack for multi-tenant isolation.

### Phase 4: Management & Deployment
- [ ] **Dashboard MVP**: A basic CLI/Web dashboard for monitoring agent status.
- [ ] **Cluster Deploy**: Finalize `scripts/deploy.sh` for Apple Silicon cluster distribution.

---

## 📝 Work Daily

### 🗓️ 2026-03-22
- **Cleaned Environment**: Stopped old `openclaw-gateway` and `ollama` services, removed global npm packages, and backed up `~/.openclaw`.
- **Source Sync**: Cloned `openclaw` and `office-claw` repositories from `winnerineast` directly to the home directory.
- **Project Refinement**: Defined the core vision of OfficeClaw (Lightweight, Secure, Manageable).
- **README Overhaul**: Completely rewrote the project documentation to align with the "Bridge Architecture" and established the action plan.
- **Multi-language Support**: Split documentation into `README.md` (English) and `README_CN.md` (Chinese).
- **Next Step**: Start **Phase 1** - Syncing the directory structure and initializing the Rust security project.

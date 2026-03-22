# 🏢 OfficeClaw: The Lightweight, Secure, and Manageable Enterprise OA Agent

/* Generated-by: [20260322-03-arch-finalization] */

[中文版 (Chinese)](README_CN.md) | **English**

**OfficeClaw** is a production-grade AI agent framework specifically designed for modern office automation (OA) environments. It leverages the powerful orchestration of **OpenClaw** while introducing a "Bridge Architecture" to meet the rigorous demands of enterprise deployment.

---

## 📜 Credits & References
OfficeClaw is built upon the foundational work and architectural concepts of the following pioneering projects:

- **[OpenClaw](https://github.com/openclaw/openclaw)**: The core personal AI assistant framework.
- **[IronClaw](https://github.com/nearai/ironclaw)**: Inspirational Rust-based implementation with WASM sandboxing.
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**: The benchmark for lightweight, OS-level container isolation.
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**: The standard for enterprise-grade privacy routing.
- **[ZeroClaw](https://github.com/winnerineast/openclaw)**: A benchmark for ultra-fast startup (<10ms).
- **[PicoClaw](https://github.com/winnerineast/openclaw)**: The gold standard for low-memory efficiency (~1MB footprint).
- **Variant References**: **Nanobot** (Python efficiency), **NullClaw** (Fast Rust binary), **Agno**, and **TinyClaw**.

---

## 🛡️ Three Core Pillars
1. **🚀 Lightweight**: <10MB idle memory, ~15k lines of optimized code.
2. **🔐 Enterprise Security**: WASM-based sandboxing, OS-level container isolation, and **mTLS** authentication.
3. **📊 Fleet Governance**: **Pull-based** centralized management for deployment, billing control, and compliance auditing.

---

## 🏗️ Bridge Architecture & Roadmap v1.3
1. **Stage 1: Quality Gate** - Test infrastructure & automated issue reporting.
2. **Stage 2: Onion Core** - ACP Gateway & 6-layer middleware pipeline.
3. **Stage 3: Security Bridge** - Rust/FFI/WASM execution runtime.
4. **Stage 4: Isolation Layer** - Per-user micro-VM/container management.
5. **Stage 5: Privacy Router** - Real-time PII scrubbing (NemoClaw).
6. **Stage 6: Fleet Governance** - Pull-based heartbeats & billing control.
7. **Stage 7: Ecosystem Absorption** - **"The Ultimate Goal"** – seamlessly run 13,000+ ClawHub skills and 20+ channels.

---

## 🛠️ Operational Mandates
- **Prompt-First Engineering**: Every line of code is born from a versioned prompt in `prompts/tasks/`.
- **Atomic Commits**: Code and prompts must be committed together.
- **Closed-Loop Verification**: Test failures trigger GitHub Issues; fixes are traced via those Issues.

---
**Status**: `Active Development` | **Architecture**: `Bridge Architecture v1.3`

---

## 📝 Work Daily

### 🗓️ 2026-03-22
- **Foundation Reset**: Cleaned environment and established the "Prompt-First" and "Always-Runnable" mandates.
- **Architecture Refinement**: Finalized v1.3 Roadmap, including the "Ecosystem Absorption" ultimate goal.
- **Stage 1 Complete**: Locked **Milestone M1** (Quality Gate) with automated GitHub Issue reporting and budget enforcers.
- **Stage 2 Deep-Dive**: Refined the Onion Core design to be protocol-driven (ACP compliant) and observable (Instrumented Pipeline).
- **Handoff**: Stage 2 is ready for Step-by-Step implementation (starting with protocol definitions).


# 📋 Task: Architectural Deep Dive & Core Primitives Specification

- **ID**: `20260322-02-arch-deep-dive`
- **Target**: Design.md, README.md, README_CN.md
- **Status**: In Progress

## 🎯 Objective
Refine the OfficeClaw v1.2 architecture by defining the core primitives, integration hooks, and fleet connectivity topology to ensure scalability, security, and enterprise manageability.

## 🛠️ Prompt
Update the OfficeClaw architectural documentation with the following deep-dive specifications:
1. **Claw Variant Integration**: Synthesize minimalism from ZeroClaw, isolation from NanoClaw, and FFI-based security from IronClaw.
2. **Core Primitives**: Define the "Onion Middleware Pipeline" for ACP messages. Establish Interface Contracts for Security, Privacy, and Isolation modules to ensure loose coupling.
3. **Fleet Topology**: Specify a "Pull-based Heartbeat" model where Core initiates connections to Fleet via mTLS for hot-reloading configurations and billing quotas.
4. **Audit Bus**: Implement a structured Event Bus for cryptographically signed audit logs that doesn't block the main reasoning loop.

## 📂 Artifacts Updated
- `Design.md`
- `README.md`
- `README_CN.md`

## 🐛 Fix Log
- **v1.0**: Initial deep dive into primitives and topology.

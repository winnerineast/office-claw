# OfficeClaw: Enterprise Local OA Agent

OfficeClaw is a secure, high-performance, and localized Office Automation (OA) Agent system designed for Apple Silicon clusters. It synthesizes the best features of the "Claw" open-source family (OpenClaw, IronClaw, NanoClaw, and NemoClaw) into a unified, production-grade architecture.

The system is optimized for **Mac Studio Clusters** (4-6 nodes), supporting 5-20 concurrent enterprise users with a "Local-First" privacy mandate.

## 🚀 Purpose
- **Privacy-First Intelligence**: Processes sensitive office data (salary, contracts, internal memos) locally on Apple Silicon.
- **Enterprise Scale**: Orchestrates multiple isolated user agents across a hardware cluster.
- **Zero-Trust Security**: Executes third-party skills in a cryptographically isolated WASM sandbox within OS-level containers.

## 🏗️ Architecture & Design
OfficeClaw employs a **Bridge Architecture** with layered defense:
1.  **Inner Layer (Rust/WASM)**: Skills are compiled to native WASM Components. The Rust Security Bridge (via NAPI-RS) enforces hard memory (10MB) and CPU fuel limits.
2.  **Middle Layer (Node.js Orchestrator)**: Handles the intelligence loop and user state management.
3.  **Intelligence Layer (Privacy Router)**: A custom gateway that scrubs PII and routes high-risk tasks to local MLX models, while falling back to Gemini API for complex public reasoning.
4.  **Outer Layer (Docker Isolation)**: Each user agent runs in a dedicated container with mutually exclusive volume mounts.

## 📂 Repository Structure
```text
office-claw/
├── security/           # Rust NAPI-RS Security Bridge (WASM Runtime)
│   ├── src/            # Resource limiting & capability enforcement logic
│   └── wit/            # WebAssembly Interface Type (WIT) definitions
├── core/               # Node.js Orchestrator
│   ├── src/            # Privacy Router & Container Management
│   └── tests/          # Checkpoint-based validation suite
├── containers/         # Production-aligned Docker configurations
│   └── agent/          # Multi-stage Dockerfile for integrated agent
├── skills/             # OA Skill Library
│   ├── summarizer/     # Native Rust WASM component for text processing
│   └── dist/           # Compiled WASM binaries
├── office-claw-up.sh   # One-click deployment script
└── OFFICE_CLAW_V&V.md  # Validation & Verification roadmap
```

## 🛠️ Build Requirements
- **Hardware**: Apple Silicon (M2 Max / Mac Studio recommended).
- **Toolchain**:
  - Rust 1.85+ (Edition 2024)
  - Node.js 22+
  - Docker / OrbStack
  - `cargo-component` & `wasm-tools`

## 📦 How to Build & Deploy

### 1. Initialize the Environment
Ensure your Rust toolchain is ready:
```bash
rustup target add wasm32-wasip1
cargo install cargo-component
```

### 2. One-Click Build
Run the provided deployment script to compile the Rust Security Bridge, build the WASM skills, and package the Docker image:
```bash
./office-claw-up.sh
```

### 3. Verification (POC)
Verify the system with the 3-user concurrent load test:
```bash
node core/tests/checkpoint5-test.js
```

## 🏃 How to Run
The system uses a `ClusterManager` to distribute users. To launch the orchestrator:
```bash
cd core
node src/index.js
```
The agent will listen on its assigned container port (default 3000) and expose a `/summarize` endpoint for skill execution.

## 🛡️ Security Features
- **Redaction**: Automated scrubbing of Emails/SSNs before any cloud routing.
- **Isolation**: Docker-level filesystem isolation ensures `User_A` can never access `User_B`'s memory.
- **Resource Limits**: WASM execution is strictly capped to protect host stability.

---
**Milestone**: `milestone-1-poc` - Verified stable state for 3 concurrent users on M2 Max.

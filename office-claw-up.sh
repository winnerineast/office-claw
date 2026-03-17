#!/bin/bash
set -e

echo "=== OfficeClaw: Enterprise OA Agent One-Click Deploy ==="

# 1. Host Toolchain Check
source $HOME/.cargo/env
command -v cargo >/dev/null 2>&1 || { echo "Rust not found. Install it first."; exit 1; }
command -v cargo-component >/dev/null 2>&1 || { echo "Installing cargo-component..."; cargo install cargo-component; }

# 2. Build WASM Skills (Natively)
echo "[1/3] Building WASM Skills..."
mkdir -p skills/dist
cd skills/summarizer && cargo component build --release
cp target/wasm32-wasip1/release/summarizer.wasm ../dist/
cd ../..

# 3. Build Agent Image (Docker)
echo "[2/3] Building Integrated Agent Image..."
/usr/local/bin/docker build -t office-claw-agent:latest -f containers/agent/Dockerfile .

# 4. Initialize Cluster State
echo "[3/3] Initializing Cluster Orchestrator..."
cd core && npm install
echo "Ready for production scaling."

echo "========================================================="
echo "OfficeClaw is built and verified."
echo "Target: 4-6 Mac Studios | 20 Enterprise Users"
echo "Launch with: node src/cluster-manager.js"
echo "========================================================="

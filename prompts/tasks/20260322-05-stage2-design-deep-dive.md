# 📋 Task: Stage 2 - Onion Core Design Deep Dive

- **ID**: `20260322-05-stage2-design-deep-dive`
- **Target**: `design_doc/Stage2_OnionCore.md`
- **Status**: In Progress

## 🎯 Objective
Refine the detailed design of the **Onion Core**, ensuring full compatibility with the **Agent Client Protocol (ACP)** and establishing a high-performance, asynchronous middleware pipeline.

## 🛠️ Prompt
Perform a deep-dive design for Stage 2 with the following requirements:
1. **ACP Compatibility Audit**: Analyze the exact handshake, frame format, and error codes of OpenClaw's ACP.
2. **Middleware Orchestration**: Design the `OnionPipeline` executor. Define how context state is passed between layers (Ingress -> Fleet -> PII -> Reasoning -> WASM -> Egress).
3. **Async Safety**: Ensure the pipeline handles streaming responses (Block Streaming) and doesn't leak memory under 20+ concurrent connections.
4. **Integration Primitives**: Define the exact TypeScript interfaces for the "Hooks" that will later be fulfilled by other modules.

## 📂 Artifacts Generated
- `design_doc/Stage2_OnionCore.md`

## 🐛 Fix Log
- **v1.0**: Initial deep-dive design.

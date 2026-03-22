# 🏗️ OfficeClaw Design: Stage 2 - Onion Core (The Heart)

/* Generated-by: [20260322-05-stage2-design-deep-dive] */

## 🎯 Goal: Protocol-Compliant & Observable Core
Implement a lightweight **ACP Gateway** and an **Instrumented Onion Pipeline**. This core must be 100% compatible with `@agentclientprotocol/sdk` and support **Block Streaming** with explicit auditability.

---

## 🏗️ Architectural Refinement

### 1. Protocol Foundation (ACP Compliance)
We use the standardized Agent Client Protocol (ACP) for all external communications.
- **Message Framing**: NDJSON (Newline Delimited JSON).
- **Core Events**: `text_delta` (Streaming), `tool_call`, `status`.
- **Session Mapping**: Map ACP `sessionId` to internal `IsolationContext`.

### 2. Instrumented Onion Pipeline
The executor is no longer a "black box" but a fully observable engine.
- **Explicit Context**: Holds `PromptRequest`, `SecureVault`, and a `Telemetry` object.
- **Middleware Hooks**:
  1. `Ingress`: Parse NDJSON.
  2. `Governance`: mTLS & Quota check (Mocked).
  3. `Privacy`: Reversible PII Redaction (Vault-backed).
  4. `Reasoning`: Orchestration loop.
  5. `Security`: WASM Execution (Mocked).
  6. `Egress`: PII Restoration & NDJSON framing.
- **Observability**: High-resolution timing for each layer and built-in error boundaries.

### 3. Reversible PII Vault (Session-Bound)
- **Mapping**: `OC_REDACTED_[TYPE][ID]` (e.g., `OC_REDACTED_E1` for Email).
- **Security**: The vault is ephemeral and bound to the session lifecycle.

---

## 🛠️ Implementation Roadmap (Step-by-Step)
1. **Step 1**: Protocol Interfaces (`protocol.ts`) & Context Primitives.
2. **Step 2**: Reversible Vault implementation with multi-type PII support.
3. **Step 3**: Instrumented `OnionExecutor` with timing and error handling.
4. **Step 4**: ACP Streaming Gateway with NDJSON routing.
5. **Step 5**: Full Integration Test using `scripts/test-and-report.sh`.

---

## ✅ Stage 2 Validation (V&V)
- **Streaming Integrity**: Verify `text_delta` events flow through the pipeline.
- **Audit Accuracy**: Assert that `IPipelineResult` timing sums match total execution time.
- **Protocol Strictness**: Validate output frames against the ACP JSON Schema.

---
**Status**: `Refined Design Locked - Ready for Implementation Tomorrow`

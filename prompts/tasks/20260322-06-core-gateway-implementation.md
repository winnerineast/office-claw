# 📋 Task: Stage 2 Implementation - Onion Core & ACP Gateway

- **ID**: `20260322-06-core-gateway-implementation`
- **Target**: `core/src/`, `core/tests/`
- **Status**: In Progress

## 🎯 Objective
Implement the heart of OfficeClaw: an ACP-compatible gateway with a 6-layer asynchronous onion middleware pipeline, including reversible PII vaulting and non-blocking fleet mocks.

## 🛠️ Prompt
Implement the Onion Core following these technical specifications:
1. **Core Primitives (`core/src/types.ts`)**:
   - Define `IContext`, `IMiddleware`, and `IPipeline` as per the Stage 2 design.
   - Include `ISecureVault` for reversible PII redaction.
2. **Onion Pipeline (`core/src/pipeline.ts`)**:
   - Implement a `PipelineExecutor` that supports `use()` and `run(context)`.
   - Ensure the `next()` pattern is strictly followed for async middleware.
3. **Secure Vault (`core/src/vault.ts`)**:
   - Implement `SecureVault` using a Map for `OC_REDACTED_` ID mapping.
4. **ACP Gateway Skeleton (`core/src/gateway.ts`)**:
   - Implement a basic NDJSON parser for ACP commands (`initialize`, `prompt`).
   - Wire the gateway into the pipeline.
5. **Mocks & Adapters (`core/src/mocks/`)**:
   - Provide minimal implementations for `FleetManager` (Always-Pass), `PrivacyRouter` (Regex-based), and `SecurityBridge` (Echo response).
6. **Validation Tests (`core/tests/onion-core.test.ts`)**:
   - Test 1: Verify PII redaction -> reasoning (mock) -> restoration loop.
   - Test 2: Verify async middleware execution order.
   - Test 3: Verify ACP `initialize` handshake with Mock Fleet.

## 📂 Artifacts Generated
- `core/src/types.ts`
- `core/src/pipeline.ts`
- `core/src/vault.ts`
- `core/src/gateway.ts`
- `core/src/mocks/**`
- `core/tests/onion-core.test.ts`

## 🐛 Fix Log
- **v1.0**: Initial implementation of the core pipeline and ACP skeleton.

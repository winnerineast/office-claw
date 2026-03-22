# 🏗️ OfficeClaw Design: Stage 2 - Onion Core (The Heart)

/* Generated-by: [20260322-05-stage2-design-deep-dive] */

## 🎯 Goal
Implement a lightweight, asynchronous **ACP Gateway** and the **Onion Middleware Pipeline**. This core manages the reversible PII vault, the WASM lifecycle, and the non-blocking fleet governance.

---

## 🏗️ Architectural Components

### 1. Reversible PII Scrubber (NemoClaw Integration)
- **Mechanism**: Bidirectional mapping via a session-bound memory vault.
- **Placeholder**: `OC_REDACTED_[TYPE][ID]` (e.g., `OC_REDACTED_E1`).
- **Restoration**: Occurs only at the **Security Layer (IronClaw)** boundary or the **Egress** boundary. The Reasoning layer remains "PII-Blind".

### 2. WASM Sandbox Lifecycle (IronClaw Integration)
1. **Verify**: Signature and provenance check.
2. **Setup**: Inject capability-based WASI imports.
3. **Run**: Enforce CPU fuel and memory quotas.
4. **Scrub**: Zero-out memory buffers upon termination.
5. **Report**: Emit execution metrics to the Audit Bus.

### 3. Non-Blocking Fleet Governance
- **Optimistic Auth**: Use "Last Known Good" (LKG) policy for immediate ACP response.
- **Async Sync**: Pull fleet updates in the background to prevent network-induced deadlocks.
- **Offline Fallback**: If Fleet is unreachable, restrict tools to local-only mode.

---

## 🛠️ Internal Primitives

```typescript
/* Generated-by: [20260322-05-stage2-design-deep-dive] */

export interface ISecureVault {
  redact(input: string): string;
  restore(redacted: string): string;
  clear(): void;
}

export interface ISandboxHandle {
  id: string;
  state: 'ready' | 'running' | 'cleaning';
  resourceUsage: { cpu: number; mem: number };
}
```

---

## ✅ Stage 2 Validation (V&V)
- **Vault Integrity**: Assert that `restore(redact(text)) === text` while `text` never enters the LLM context.
- **Lifecycle Probe**: Force a WASM instance to hang and verify the core kills it and scrubs memory within 500ms.
- **Latency Test**: Assert `initialize` response time < 100ms even if Fleet Mock is set to high latency (2s).

---
**Status**: `Deep-Dive Design Finalized with HITL and Safety Guards`

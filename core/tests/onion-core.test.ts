/* Generated-by: [20260322-06-core-gateway-implementation] */
import { describe, it, expect } from 'vitest';
import { AcpGateway } from '../src/gateway.js';
import { SecureVault } from '../src/vault.js';
import { PipelineExecutor } from '../src/pipeline.js';

describe('Stage 2: Onion Core Implementation', () => {
  
  it('should verify reversible PII redaction in SecureVault', () => {
    const vault = new SecureVault();
    const original = "Contact me at alice@example.com or bob@company.com";
    const redacted = vault.redact(original);
    
    expect(redacted).toContain('OC_REDACTED_E1');
    expect(redacted).toContain('OC_REDACTED_E2');
    expect(redacted).not.toContain('alice@example.com');
    
    const restored = vault.restore(redacted);
    expect(restored).toBe(original);
  });

  it('should verify async middleware execution order in PipelineExecutor', async () => {
    const executor = new PipelineExecutor();
    const sequence: string[] = [];

    executor.use({
      name: 'Layer1',
      execute: async (ctx, next) => {
        sequence.push('L1-In');
        await next();
        sequence.push('L1-Out');
      }
    });

    executor.use({
      name: 'Layer2',
      execute: async (ctx, next) => {
        sequence.push('L2-In');
        await next();
        sequence.push('L2-Out');
      }
    });

    const mockContext: any = {};
    await executor.run(mockContext);

    expect(sequence).toEqual(['L1-In', 'L2-In', 'L2-Out', 'L1-Out']);
  });

  it('should run full message loop through AcpGateway', async () => {
    const gateway = new AcpGateway();
    const message = "My secret email is secret@office.com";
    const response = await gateway.handlePrompt("user-1", "session-1", message);
    
    // Response should be restored by the Egress layer
    expect(response).toContain("Echo: My secret email is secret@office.com");
  });
});

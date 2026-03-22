/* Generated-by: [20260322-06-core-gateway-implementation] */
import { IContext, IPipeline, ISecureVault } from './types.js';
import { PipelineExecutor } from './pipeline.js';
import { SecureVault } from './vault.js';

export class AcpGateway {
  private pipeline = new PipelineExecutor();
  private vault: ISecureVault = new SecureVault();

  constructor() {
    this.setupDefaultPipeline();
  }

  private setupDefaultPipeline() {
    // 1. Ingress Middleware (Internal)
    this.pipeline.use({
      name: 'Governance',
      execute: async (ctx, next) => {
        // Mock Governance Check
        ctx.isAuthorized = true;
        await next();
      }
    });

    this.pipeline.use({
      name: 'Privacy',
      execute: async (ctx, next) => {
        ctx.redactedMessage = this.vault.redact(ctx.message);
        await next();
      }
    });

    this.pipeline.use({
      name: 'Reasoning',
      execute: async (ctx, next) => {
        // Mock Reasoning: Simple Echo
        ctx.response = `Echo: ${ctx.redactedMessage}`;
        await next();
      }
    });

    this.pipeline.use({
      name: 'Egress',
      execute: async (ctx, next) => {
        // Restore PII before sending back to user
        if (ctx.response) {
          ctx.response = this.vault.restore(ctx.response);
        }
        await next();
      }
    });
  }

  async handlePrompt(userId: string, sessionId: string, message: string): Promise<string> {
    const context: IContext = {
      userId,
      sessionId,
      message,
      metadata: {},
      isAuthorized: false,
      budgetExceeded: false
    };

    await this.pipeline.run(context);
    return context.response || '';
  }
}

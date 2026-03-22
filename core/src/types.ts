/* Generated-by: [20260322-06-core-gateway-implementation] */

export interface IContext {
  userId: string;
  sessionId: string;
  message: string;
  redactedMessage?: string;
  response?: string;
  metadata: Record<string, any>;
  isAuthorized: boolean;
  budgetExceeded: boolean;
}

export type NextFunction = () => Promise<void>;

export interface IMiddleware {
  name: string;
  execute(context: IContext, next: NextFunction): Promise<void>;
}

export interface ISecureVault {
  redact(input: string): string;
  restore(redacted: string): string;
  clear(): void;
}

export interface IPrivacyRouter {
  redact(input: string, vault: ISecureVault): string;
}

export interface IFleetManager {
  checkPolicy(context: IContext): Promise<boolean>;
  getQuota(userId: string): Promise<number>;
}

export interface ISecurityBridge {
  executeTool(toolId: string, args: any): Promise<any>;
}

/* Generated-by: [20260322-06-core-gateway-implementation] */
import { ISecurityBridge } from '../types.js';

export class MockSecurityBridge implements ISecurityBridge {
  async executeTool(toolId: string, args: any): Promise<any> {
    return { result: `Mock execution of ${toolId}`, args };
  }
}

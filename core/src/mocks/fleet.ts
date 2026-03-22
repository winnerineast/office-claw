/* Generated-by: [20260322-06-core-gateway-implementation] */
import { IFleetManager, IContext } from '../types.js';

export class MockFleetManager implements IFleetManager {
  async checkPolicy(context: IContext): Promise<boolean> {
    return true; // Always pass for mock
  }
  async getQuota(userId: string): Promise<number> {
    return 100.0; // Fixed quota for mock
  }
}

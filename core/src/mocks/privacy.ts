/* Generated-by: [20260322-06-core-gateway-implementation] */
import { IPrivacyRouter, ISecureVault } from '../types.js';

export class MockPrivacyRouter implements IPrivacyRouter {
  redact(input: string, vault: ISecureVault): string {
    return vault.redact(input);
  }
}

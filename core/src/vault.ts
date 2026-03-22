/* Generated-by: [20260322-06-core-gateway-implementation] */
import { ISecureVault } from './types.js';

export class SecureVault implements ISecureVault {
  private vault = new Map<string, string>();
  private reverseVault = new Map<string, string>();
  private counter = 0;

  redact(input: string): string {
    // Simple email regex for the probe implementation
    const emailRegex = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/g;
    
    return input.replace(emailRegex, (match) => {
      if (this.reverseVault.has(match)) {
        return this.reverseVault.get(match)!;
      }
      
      const id = `OC_REDACTED_E${++this.counter}`;
      this.vault.set(id, match);
      this.reverseVault.set(match, id);
      return id;
    });
  }

  restore(redacted: string): string {
    let output = redacted;
    for (const [id, original] of this.vault.entries()) {
      output = output.split(id).join(original);
    }
    return output;
  }

  clear(): void {
    this.vault.clear();
    this.reverseVault.clear();
    this.counter = 0;
  }
}

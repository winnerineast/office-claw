/**
 * OfficeClaw Privacy Router (Refined)
 * 
 * Separates "High Risk" (Always Local) from "Moderate Risk" (Scrub then Cloud).
 */

class PrivacyRouter {
  constructor(config = {}) {
    this.highRiskKeywords = config.highRiskKeywords || [
      'salary', 'contract', 'internal', 'private', 'password', 
      'secret', 'ssn', 'tax', 'revenue', 'budget'
    ];
    this.highRiskPatterns = [
      /\b\d{3}-\d{2}-\d{4}\b/g // SSN
    ];
    this.moderateRiskPatterns = [
      /\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/gi, // Email
      /\b(?:\d[ -]*?){13,16}\b/g // Credit Card
    ];
  }

  classify(prompt) {
    const lowerPrompt = prompt.toLowerCase();
    
    // Rule 1: High-Risk Keyword or Pattern -> MUST BE LOCAL
    if (this.highRiskKeywords.some(kw => lowerPrompt.includes(kw))) return 'local';
    if (this.highRiskPatterns.some(regex => regex.test(prompt))) return 'local';

    return 'cloud';
  }

  scrub(prompt) {
    let scrubbed = prompt;
    // Scrub moderate risk patterns
    this.moderateRiskPatterns.forEach(regex => {
      scrubbed = scrubbed.replace(regex, '[REDACTED]');
    });
    // Double-check high-risk patterns are also scrubbed just in case
    this.highRiskPatterns.forEach(regex => {
      scrubbed = scrubbed.replace(regex, '[REDACTED]');
    });
    return scrubbed;
  }

  async route(prompt) {
    const destination = this.classify(prompt);
    
    if (destination === 'local') {
      return {
        destination: 'local',
        payload: prompt,
        reason: 'High-risk office data detected. Local only.'
      };
    }

    return {
      destination: 'cloud',
      payload: this.scrub(prompt),
      reason: 'Safe for cloud reasoning after scrubbing.'
    };
  }
}

module.exports = PrivacyRouter;

/* Generated-by: [20260322-03-quality-gate-infra] */
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['tests/**/*.test.ts'],
    // Hexagonal Defense: Monitor memory usage after each test
    teardownTimeout: 1000,
    onConsoleLog(log) {
      if (log.includes('Memory Usage')) return false;
    },
  },
});

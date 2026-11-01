import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests/browser', timeout: 40000, expect: { timeout: 10000 },
  retries: 0, workers: 1, reporter: [['list'], ['json', { outputFile: '.cache/browser-results.json' }]],
  use: { baseURL: process.env.PACKET_BASE_URL || 'http://127.0.0.1:4177/PACKET/', acceptDownloads: true, trace: 'retain-on-failure' },
  webServer: process.env.PACKET_BASE_URL ? undefined : { command: 'python -B tools/serve-pages.py', url: 'http://127.0.0.1:4177/PACKET/', reuseExistingServer: false }
});

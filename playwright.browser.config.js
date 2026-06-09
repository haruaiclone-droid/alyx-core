import { devices, defineConfig } from "@playwright/test";

const baseURL = process.env.ALYX_BASE_URL || "http://127.0.0.1:38888";

export default defineConfig({
  testDir: "./.github/browser-smoke",
  timeout: 15000,
  use: {
    baseURL,
    headless: true,
    ignoreHTTPSErrors: true,
  },
  reporter: [["list"]],
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },
  ],
});

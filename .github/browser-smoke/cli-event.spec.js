import { test, expect } from "@playwright/test";

test("CLI dist event bridge handles click and keyboard input", async ({ page, baseURL }) => {
  const counts = {
    click: 0,
    keydown: 0,
  };
  const enterPayload = {
    keydown: 0,
  };

  page.on("requestfinished", (request) => {
    if (!request.url().endsWith("/__alyx_event")) {
      return;
    }

    if (request.method() !== "POST") {
      return;
    }

    const payload = request.postDataJSON();
    if (payload?.type === "click") {
      counts.click += 1;
    }
    if (payload?.type === "keydown") {
      counts.keydown += 1;
      if (payload?.key === "Enter") {
        enterPayload.keydown += 1;
      }
    }
  });

  await page.goto(baseURL);
  await expect(page.locator("text=count: 0")).toBeVisible();
  await expect(page.locator("text=+")).toBeVisible();
  await expect(page.locator("text=reset")).toBeVisible();

  const increment = page.locator("text=+");
  await increment.click();

  await page.locator("body").click();
  await page.keyboard.press("Enter");
  await page.waitForTimeout(300);

  expect(counts.click).toBeGreaterThanOrEqual(1);
  expect(counts.keydown).toBeGreaterThanOrEqual(1);
  expect(enterPayload.keydown).toBeGreaterThanOrEqual(1);
});

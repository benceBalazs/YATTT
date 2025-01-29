import { test, expect } from '@playwright/test';

test('should display the dashboard with charts', async ({ page }) => {
    await page.goto('http://localhost:4200/dashboard');
    await expect(page.locator('#attendanceChart')).toBeVisible();
    await expect(page.locator('#cardUsageChart')).toBeVisible();
});

test('should load data into charts', async ({ page }) => {
    await page.route('**/api/dashboard-data', async (route) => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                attendance: [10, 20, 30, 40],
                cardUsage: [5, 15, 25, 35]
            }),
        });
    });
    await page.goto('http://localhost:4200/dashboard');
    const chartData = await page.locator('#attendanceChart').textContent();
    expect(chartData).not.toBeNull();
    const cardData = await page.locator('#cardUsageChart').textContent();
    expect(cardData).not.toBeNull();
});

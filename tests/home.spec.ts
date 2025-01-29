import { test, expect } from '@playwright/test';

test('should display the table with correct headers', async ({ page }) => {
    await page.goto('http://localhost:4200/home');
    await expect(page.locator('table')).toBeVisible();
    const headers = await page.locator('th').allTextContents();
    expect(headers).toEqual([
        'Check in time', 'Check out time', 'Duration', 'Card', 'Course'
    ]);
});

test('should sort check-in time column', async ({ page }) => {
    await page.goto('http://localhost:4200/home');
    await page.click('th:has-text("Course")');
    const firstCourse = await page.locator('td:nth-child(1)').textContent();
    await page.click('th:has-text("Course")');
    const firstCourseDesc = await page.locator('td:nth-child(1)').textContent();
    expect(firstCourse).not.toBe(firstCourseDesc);
});


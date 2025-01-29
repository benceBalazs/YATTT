import { test, expect } from '@playwright/test';

test('should log in successfully with valid credentials', async ({ page }) => {
    await page.goto('http://localhost:4200/login');
    await page.fill('input[formControlName="username"]', 'superuser');
    await page.fill('input[formControlName="password"]', '123456789');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/home/);
});

test('should show error message on invalid login', async ({ page }) => {
    await page.goto('http://localhost:4200/login');
    await page.fill('input[formControlName="username"]', 'wrongUser');
    await page.fill('input[formControlName="password"]', 'WrongPassword');
    await page.click('button[type="submit"]');
    const errorMessage = await page.textContent('.errorMessage');
    expect(errorMessage).toContain('Invalid username or password');
});

test('login button should be disabled if fields are empty', async ({ page }) => {
    await page.goto('http://localhost:4200/login');
    const isDisabled = await page.getAttribute('button[type="submit"]', 'disabled');
    expect(isDisabled).not.toBeNull();
    await page.fill('input[formControlName="username"]', 'testUser');
    expect(await page.getAttribute('button[type="submit"]', 'disabled')).not.toBeNull();
    await page.fill('input[formControlName="password"]', 'TestPassword123');
    expect(await page.getAttribute('button[type="submit"]', 'disabled')).toBeNull();
});

test('should register successfully with valid credentials', async ({ page }) => {
    await page.goto('http://localhost:4200/login');
    await page.fill('input[formControlName="username"]', 'newUser');
    await page.fill('input[formControlName="password"]', 'NewPassword123');
    await page.click('button[type="submit"]:has-text("Register")');
    await expect(page).toHaveURL(/dashboard/);
});

test('should show error message if password is too short', async ({ page }) => {
    await page.goto('http://localhost:4200/login');
    await page.fill('input[formControlName="username"]', 'newUser');
    await page.fill('input[formControlName="password"]', '12345');
    const errorMessage = await page.textContent('.errorMessage');
    expect(errorMessage).toContain('Password must be at least 8 characters long.');
});



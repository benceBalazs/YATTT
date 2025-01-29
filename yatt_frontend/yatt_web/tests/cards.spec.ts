import { test, expect } from '@playwright/test';

test.describe('Cards Component', () => {
    test('should add a new card', async ({ page }) => {
        // Navigate to the page
        await page.goto('http://localhost:4200/cards');

        // Click the "Add Card" button
        await page.click('button:has-text("Add Card")');

        // Fill out the form
        await page.fill('input#name', 'Test Card');
        await page.fill('input#tag_id', '12345');

        // Submit the form
        await page.click('button[type="submit"]');

        // Verify the card appears in the table
        const cardName = await page.textContent('table tr:last-child td:first-child');
        const cardTag = await page.textContent('table tr:last-child td:nth-child(2)');
        expect(cardName).toBe('Test Card');
        expect(cardTag).toBe('12345');
    });
});

test('should edit an existing card', async ({ page }) => {
    await page.goto('http://localhost:4200/cards');

    // Open edit modal for the first card
    await page.click('table tr:nth-child(1) button:has-text("Edit")');

    // Change the card name
    await page.fill('input#name', 'Updated Card Name');

    // Submit the form
    await page.click('button[type="submit"]');

    // Verify the updated card name is displayed
    const updatedName = await page.textContent('table tr:nth-child(1) td:first-child');
    expect(updatedName).toBe('Updated Card Name');
});

test('should delete a card', async ({ page }) => {
    await page.goto('http://localhost:4200/cards');

    // Count the initial number of rows
    const initialRowCount = await page.locator('table tr').count();

    // Delete the first card
    await page.click('table tr:nth-child(1) button:has-text("Delete")');

    // Verify the row count decreases
    const updatedRowCount = await page.locator('table tr').count();
    expect(updatedRowCount).toBe(initialRowCount - 1);
});


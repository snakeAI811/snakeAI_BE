import { test, expect } from '@playwright/test';

// Mock wallet connection for E2E tests
test.beforeEach(async ({ page }) => {
  // Mock the Solana wallet adapter
  await page.addInitScript(() => {
    (window as any).solana = {
      isPhantom: true,
      connect: async () => ({
        publicKey: {
          toString: () => '11111111111111111111111111111111'
        }
      }),
      disconnect: async () => {},
      on: (event: string, handler: Function) => {},
      request: async (method: string, params: any) => {
        if (method === 'connect') {
          return {
            publicKey: {
              toString: () => '11111111111111111111111111111111'
            }
          };
        }
        return {};
      }
    };
  });
});

test.describe('Patron Framework E2E Tests', () => {
  test('complete patron application workflow', async ({ page }) => {
    // Start the frontend development server first
    await page.goto('http://localhost:3000/claim/test-user-123');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Connect wallet
    await page.click('text=Connect Wallet');
    await page.click('text=Phantom');
    
    // Wait for wallet connection
    await page.waitForSelector('text=Connected');

    // Select patron role
    await page.click('text=Patron');
    await page.waitForSelector('text=10% enhanced rewards');

    // Check if user claim needs initialization
    const initButton = await page.locator('text=Initialize Claim Account');
    if (await initButton.isVisible()) {
      await initButton.click();
      await page.waitForSelector('text=Initializing...');
      await page.waitForSelector('text=Initialize Claim Account', { state: 'hidden' });
    }

    // Apply for patron status
    await page.click('text=Apply for Patron');
    
    // Fill patron application form
    await page.fill('input[placeholder*="wallet creation"]', '120');
    await page.fill('input[placeholder*="engagement score"]', '75');
    await page.fill('textarea[placeholder*="commitment"]', 'I am committed to the Snake AI community and will contribute actively to governance and platform development.');
    
    // Submit application
    await page.click('text=Submit Application');
    await page.waitForSelector('text=Submitting...');
    await page.waitForSelector('text=Application submitted successfully');

    // Claim tokens
    await page.click('text=Claim');
    await page.waitForSelector('text=Claiming...');
    await page.waitForSelector('text=Claimed Successfully!');

    // Verify final state
    await expect(page.locator('text=Claimed Successfully!')).toBeVisible();
    await expect(page.locator('text=Patron')).toBeVisible();
  });

  test('role selection and benefits display', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Test staker role
    await page.click('text=Staker');
    await expect(page.locator('text=5% staking rewards')).toBeVisible();
    await expect(page.locator('text=Priority support')).toBeVisible();
    await expect(page.locator('text=Lock Period: 3 months')).toBeVisible();

    // Test patron role
    await page.click('text=Patron');
    await expect(page.locator('text=10% enhanced rewards')).toBeVisible();
    await expect(page.locator('text=Governance voting')).toBeVisible();
    await expect(page.locator('text=Lock Period: 6 months')).toBeVisible();

    // Test normal user role
    await page.click('text=Normal User');
    await expect(page.locator('text=Token claiming')).toBeVisible();
    await expect(page.locator('text=Lock Period: No lock')).toBeVisible();
  });

  test('mining statistics display', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Wait for mining data to load
    await page.waitForSelector('text=Phase 1 Mining:');
    await page.waitForSelector('text=Phase 2 Mining:');
    await page.waitForSelector('text=Total Mining:');

    // Verify mining statistics are displayed
    await expect(page.locator('text=Phase 1 Mining:')).toBeVisible();
    await expect(page.locator('text=Phase 2 Mining:')).toBeVisible();
    await expect(page.locator('text=Total Mining:')).toBeVisible();
    await expect(page.locator('text=Current Phase:')).toBeVisible();
  });

  test('error handling and retry functionality', async ({ page }) => {
    // Mock network failure
    await page.route('**/api/user/*/mining_status', route => {
      route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Internal server error' })
      });
    });

    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Wait for error message
    await page.waitForSelector('text=Failed to fetch claim data');
    await expect(page.locator('text=Failed to fetch claim data')).toBeVisible();

    // Test retry functionality
    await expect(page.locator('text=Retry')).toBeVisible();
    
    // Remove the mock to allow retry to succeed
    await page.unroute('**/api/user/*/mining_status');
    
    await page.click('text=Retry');
    await page.waitForSelector('text=Claim Summary');
  });

  test('wallet connection flow', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Should show wallet connection prompt
    await expect(page.locator('text=Connect Your Wallet')).toBeVisible();
    await expect(page.locator('text=Please connect your wallet to claim your Snake AI tokens')).toBeVisible();

    // Connect wallet
    await page.click('text=Connect Wallet');
    
    // Should show wallet options
    await expect(page.locator('text=Phantom')).toBeVisible();
    await expect(page.locator('text=Solflare')).toBeVisible();

    // Select Phantom wallet
    await page.click('text=Phantom');
    
    // Should redirect to main claim interface
    await page.waitForSelector('text=Claim Summary');
    await expect(page.locator('text=Claim Summary')).toBeVisible();
  });

  test('responsive design on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Connect wallet
    await page.click('text=Connect Wallet');
    await page.click('text=Phantom');
    
    // Verify mobile layout
    await page.waitForSelector('text=Claim Summary');
    
    // Check that components are properly stacked on mobile
    const roleSelector = page.locator('text=Select Your Role');
    const claimInterface = page.locator('text=Claim Tokens');
    
    await expect(roleSelector).toBeVisible();
    await expect(claimInterface).toBeVisible();
    
    // Verify mobile-friendly interactions
    await page.click('text=Staker');
    await expect(page.locator('text=5% staking rewards')).toBeVisible();
  });

  test('accessibility features', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Check for proper heading structure
    await expect(page.locator('h1')).toContainText('Connect Your Wallet');
    
    // Check for proper form labels
    await page.click('text=Connect Wallet');
    await page.click('text=Phantom');
    await page.waitForSelector('text=Claim Summary');
    
    // Check ARIA labels and roles
    const claimButton = page.locator('button:has-text("Claim")').first();
    await expect(claimButton).toBeVisible();
    
    // Check keyboard navigation
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
  });

  test('patron application validation', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Connect wallet and navigate to patron application
    await page.click('text=Connect Wallet');
    await page.click('text=Phantom');
    await page.waitForSelector('text=Claim Summary');
    
    await page.click('text=Patron');
    await page.click('text=Apply for Patron');

    // Test validation errors
    await page.fill('input[placeholder*="wallet creation"]', '30');
    await page.fill('input[placeholder*="engagement score"]', '20');
    await page.click('text=Submit Application');

    // Should show validation errors
    await expect(page.locator('text=Wallet must be at least 90 days old')).toBeVisible();
    await expect(page.locator('text=Community score must be at least 50')).toBeVisible();

    // Test successful submission
    await page.fill('input[placeholder*="wallet creation"]', '120');
    await page.fill('input[placeholder*="engagement score"]', '75');
    await page.fill('textarea[placeholder*="commitment"]', 'I am committed to the community');
    
    await page.click('text=Submit Application');
    await page.waitForSelector('text=Application submitted successfully');
  });

  test('transaction flow and confirmations', async ({ page }) => {
    await page.goto('http://localhost:3000/claim/test-user-123');
    await page.waitForLoadState('networkidle');

    // Connect wallet
    await page.click('text=Connect Wallet');
    await page.click('text=Phantom');
    await page.waitForSelector('text=Claim Summary');

    // Select staker role
    await page.click('text=Staker');

    // Initialize if needed
    const initButton = page.locator('text=Initialize Claim Account');
    if (await initButton.isVisible()) {
      await initButton.click();
      await page.waitForSelector('text=Initializing...');
      await page.waitForSelector('text=Initialize Claim Account', { state: 'hidden' });
    }

    // Claim tokens
    await page.click('text=Claim');
    await page.waitForSelector('text=Claiming...');
    
    // Verify transaction completion
    await page.waitForSelector('text=Claimed Successfully!', { timeout: 30000 });
    await expect(page.locator('text=Claimed Successfully!')).toBeVisible();
    
    // Verify button is disabled after claiming
    const claimButton = page.locator('button:has-text("Claimed Successfully!")');
    await expect(claimButton).toBeDisabled();
  });
});

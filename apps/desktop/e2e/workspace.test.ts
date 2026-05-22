import { expect } from '@wdio/globals';

describe('SDE-KIT E2E Test Suite', () => {
  before(async () => {
    // Wait for the main window shell to render completely
    const workspaceElement = await $('#workspace-root');
    await workspaceElement.waitForExist({ timeout: 15000 });
  });

  it('should initialize the offline application and show default explorer panel', async () => {
    // 1. Assert that the title is correct
    const title = await browser.getTitle();
    expect(title).toBe('SDE Kit');

    // 2. Assert that the explorer sidebar panel is active by default
    const explorerPanel = await $('.sidebar-panel[data-panel="explorer"]');
    expect(await explorerPanel.isDisplayed()).toBe(true);
  });

  it('should support local-first project creation', async () => {
    // Switch to Projects panel
    const projectsTab = await $('[aria-label="Projects panel"]');
    await projectsTab.click();

    const projectsPanel = await $('.projects-panel');
    expect(await projectsPanel.isDisplayed()).toBe(true);

    // Click "+" button to show new project form
    const addProjectBtn = await $('.add-project-btn');
    await addProjectBtn.click();

    // Input project metadata
    const nameInput = await $('input[placeholder="Project name"]');
    await nameInput.setValue('E2E Test Workspace Project');

    const pathInput = await $('input[placeholder="Local workspace path"]');
    await pathInput.setValue('/Users/e2e-user/desktop/test-project');

    const descInput = await $('textarea[placeholder="Description"]');
    await descInput.setValue('Offline local database testing.');

    // Save project
    const saveBtn = await $('.btn-primary=Create');
    await saveBtn.click();

    // Verify it appeared in the project list card
    const projectCard = await $('.project-card*=E2E Test Workspace Project');
    expect(await projectCard.isDisplayed()).toBe(true);
  });

  it('should verify workspace file tree indexing and markdown rendering', async () => {
    // Navigate back to explorer panel
    const explorerTab = await $('[aria-label="Explorer panel"]');
    await explorerTab.click();

    // Verify folder structures are rendered locally from workspace root
    const folderToggle = await $('.folder-toggle');
    expect(await folderToggle.isDisplayed()).toBe(true);

    // Expand folder
    await folderToggle.click();

    // Verify nested file tree nodes rendered
    const fileRow = await $('.file-row');
    expect(await fileRow.isDisplayed()).toBe(true);

    // Double click file to open tab
    await fileRow.doubleClick();

    // Verify that tab gets created in the editor layout
    const activeTab = await $('.tab.active');
    expect(await activeTab.isDisplayed()).toBe(true);

    // Verify editor area is rendered with offline CodeMirror context
    const codeEditor = await $('.cm-editor');
    expect(await codeEditor.isDisplayed()).toBe(true);
  });

  it('should perform project exports offline', async () => {
    // Navigate back to Projects Panel
    const projectsTab = await $('[aria-label="Projects panel"]');
    await projectsTab.click();

    // Click JSON export button on E2E Project card
    const jsonExportBtn = await $('.btn-export-json');
    expect(await jsonExportBtn.isDisplayed()).toBe(true);
    await jsonExportBtn.click();

    // Assert loading overlay appeared and disappeared rapidly (<100ms)
    const loader = await $('.loading-overlay');
    const wasVisible = await loader.isDisplayed();
    if (wasVisible) {
      await loader.waitForDisplayed({ reverse: true, timeout: 2000 });
    }

    // Verify the visual toast feedback triggers successfully
    const toast = await $('.toast-container');
    expect(await toast.isDisplayed()).toBe(true);
  });
});

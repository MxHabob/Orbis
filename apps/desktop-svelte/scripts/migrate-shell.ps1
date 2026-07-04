$src = "c:\Users\YZLPN3\Downloads\new\Orbis\apps\desktop-svelte\src\lib\components\layout"
$dest = "c:\Users\YZLPN3\Downloads\new\Orbis\apps\desktop-svelte\src\lib\shell"
New-Item -ItemType Directory -Force -Path $dest | Out-Null

$mappings = @(
  @("AppChrome.svelte", "app-chrome.svelte"),
  @("AppSidebar.svelte", "app-sidebar.svelte"),
  @("AskOrbisSidebar.svelte", "ask-orbis-sidebar.svelte"),
  @("DetailPageHeader.svelte", "detail-page-header.svelte"),
  @("MainViewport.svelte", "main-viewport.svelte"),
  @("OnboardingPopover.svelte", "onboarding-popover.svelte"),
  @("OrgSidebarSwitcher.svelte", "org-sidebar-switcher.svelte"),
  @("Resizer.svelte", "resizer.svelte"),
  @("SashLayer.svelte", "sash-layer.svelte"),
  @("SearchModal.svelte", "search-modal.svelte"),
  @("SecondaryMenu.svelte", "secondary-menu.svelte"),
  @("SidebarNav.svelte", "sidebar-nav.svelte"),
  @("TitleBar.svelte", "title-bar.svelte"),
  @("TitleBreadcrumbs.svelte", "title-breadcrumbs.svelte"),
  @("UpgradeDialog.svelte", "upgrade-dialog.svelte"),
  @("UserSidebarMenu.svelte", "user-sidebar-menu.svelte"),
  @("WhatsNewPopover.svelte", "whats-new-popover.svelte")
)

foreach ($m in $mappings) {
  $source = Join-Path $src $m[0]
  $target = Join-Path $dest $m[1]
  if (Test-Path $source) {
    Copy-Item $source $target -Force
    Write-Host "Copied: $($m[0]) -> $($m[1])"
  } else {
    Write-Host "MISSING: $($m[0])"
  }
}

Write-Host "`nDone! Copied $($mappings.Count) shell files."

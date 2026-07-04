# Phase 1: Update all imports from $lib/components/layout/* to $lib/shell/*
# This script updates all files in src/routes/ and src/lib/

$rootDir = "c:\Users\YZLPN3\Downloads\new\Orbis\apps\desktop-svelte\src"

$importMappings = @(
  @('$lib/components/layout/AppChrome.svelte', '$lib/shell/app-chrome.svelte'),
  @('$lib/components/layout/AppSidebar.svelte', '$lib/shell/app-sidebar.svelte'),
  @('$lib/components/layout/AskOrbisSidebar.svelte', '$lib/shell/ask-orbis-sidebar.svelte'),
  @('$lib/components/layout/DetailPageHeader.svelte', '$lib/shell/detail-page-header.svelte'),
  @('$lib/components/layout/MainViewport.svelte', '$lib/shell/main-viewport.svelte'),
  @('$lib/components/layout/OnboardingPopover.svelte', '$lib/shell/onboarding-popover.svelte'),
  @('$lib/components/layout/OrgSidebarSwitcher.svelte', '$lib/shell/org-sidebar-switcher.svelte'),
  @('$lib/components/layout/Resizer.svelte', '$lib/shell/resizer.svelte'),
  @('$lib/components/layout/SashLayer.svelte', '$lib/shell/sash-layer.svelte'),
  @('$lib/components/layout/SearchModal.svelte', '$lib/shell/search-modal.svelte'),
  @('$lib/components/layout/SecondaryMenu.svelte', '$lib/shell/secondary-menu.svelte'),
  @('$lib/components/layout/SidebarNav.svelte', '$lib/shell/sidebar-nav.svelte'),
  @('$lib/components/layout/TitleBar.svelte', '$lib/shell/title-bar.svelte'),
  @('$lib/components/layout/TitleBreadcrumbs.svelte', '$lib/shell/title-breadcrumbs.svelte'),
  @('$lib/components/layout/UpgradeDialog.svelte', '$lib/shell/upgrade-dialog.svelte'),
  @('$lib/components/layout/UserSidebarMenu.svelte', '$lib/shell/user-sidebar-menu.svelte'),
  @('$lib/components/layout/WhatsNewPopover.svelte', '$lib/shell/whats-new-popover.svelte')
)

$count = 0
Get-ChildItem $rootDir -Recurse -Include "*.svelte","*.ts" | Where-Object { $_.FullName -notmatch "node_modules|\.svelte-kit" } | ForEach-Object {
  $file = $_
  $content = Get-Content $file.FullName -Raw
  if ($null -eq $content) { return }
  $changed = $false
  foreach ($pair in $importMappings) {
    $old = $pair[0]
    $new = $pair[1]
    if ($content.Contains($old)) {
      $content = $content.Replace($old, $new)
      $changed = $true
    }
  }
  if ($changed) {
    Set-Content $file.FullName $content -NoNewline
    $count++
    Write-Host "Updated: $($file.FullName.Replace($rootDir, 'src'))"
  }
}
Write-Host "`nTotal files updated: $count"

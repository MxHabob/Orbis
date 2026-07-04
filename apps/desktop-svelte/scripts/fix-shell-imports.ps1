$shellDir = "c:\Users\YZLPN3\Downloads\new\Orbis\apps\desktop-svelte\src\lib\shell"
$replacements = @(
  @("./AppSidebar.svelte", "./app-sidebar.svelte"),
  @("./AppChrome.svelte", "./app-chrome.svelte"),
  @("./AskOrbisSidebar.svelte", "./ask-orbis-sidebar.svelte"),
  @("./DetailPageHeader.svelte", "./detail-page-header.svelte"),
  @("./MainViewport.svelte", "./main-viewport.svelte"),
  @("./OnboardingPopover.svelte", "./onboarding-popover.svelte"),
  @("./OrgSidebarSwitcher.svelte", "./org-sidebar-switcher.svelte"),
  @("./Resizer.svelte", "./resizer.svelte"),
  @("./SashLayer.svelte", "./sash-layer.svelte"),
  @("./SearchModal.svelte", "./search-modal.svelte"),
  @("./SecondaryMenu.svelte", "./secondary-menu.svelte"),
  @("./SidebarNav.svelte", "./sidebar-nav.svelte"),
  @("./TitleBar.svelte", "./title-bar.svelte"),
  @("./TitleBreadcrumbs.svelte", "./title-breadcrumbs.svelte"),
  @("./UpgradeDialog.svelte", "./upgrade-dialog.svelte"),
  @("./UserSidebarMenu.svelte", "./user-sidebar-menu.svelte"),
  @("./WhatsNewPopover.svelte", "./whats-new-popover.svelte")
)

Get-ChildItem $shellDir -Filter "*.svelte" | ForEach-Object {
  $file = $_
  $content = Get-Content $file.FullName -Raw
  $changed = $false
  foreach ($pair in $replacements) {
    $old = $pair[0]
    $new = $pair[1]
    if ($content.Contains($old)) {
      $content = $content.Replace($old, $new)
      $changed = $true
    }
  }
  if ($changed) {
    Set-Content $file.FullName $content -NoNewline
    Write-Host "Fixed imports in: $($file.Name)"
  }
}
Write-Host "Done fixing shell internal imports."

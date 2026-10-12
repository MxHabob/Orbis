<!--
@component AppChrome
The outermost layout shell for the authenticated application.
Follows GitButler's "Chrome Shell" pattern:

  ┌──────────────────────────────────┐
  │           AppHeader              │
  ├──┬───────────────────────────────┤
  │  │                               │
  │S │       chrome-content          │
  │i │     (page via slot)           │
  │d │                               │
  │e │                               │
  │b │                               │
  │a │                               │
  │r │                               │
  │  │                               │
  └──┴───────────────────────────────┘
-->
<script lang="ts">
import AppSidebar from "./app-sidebar.svelte";
import LicenseBanner from "./license-banner.svelte";
import { licenseStore, uiStore } from "$lib/stores";
import type { Snippet } from "svelte";
import { slide } from "svelte/transition";
import { page } from "$app/state";
import AskOrbisSidebar from "$lib/features/ai/views/sidebar.svelte";
import { useQueryClient } from "@tanstack/svelte-query";
import LicenseFeatureGate from "./license-feature-gate.svelte";
import type { EntitlementKey } from "$lib/api/types";

let {
  orgId,
  orgName,
  licenseReady,
  children,
}: {
  orgId: string;
  orgName?: string;
  licenseReady: boolean;
  children: Snippet;
} = $props();

const queryClient = useQueryClient();
let askOrbisWidth = $state(25);

const requiredFeature = $derived.by((): EntitlementKey | null => {
  const path = page.url.pathname;
  if (/(^|\/)students(\/|$)/.test(path)) return "students";
  if (path.includes("/ai-center")) return "ai_grading";
  if (path.includes("/academics/exams")) return "exam_tools";
  if (path.includes("/settings/branches") || path.includes("/settings/units")) {
    return "multi_branch";
  }
  if (
    (path.includes("/reports") && !path.includes("/finance/reports")) ||
    path.includes("/analytics")
  ) {
    return "campus_analytics";
  }
  if (path.includes("/parent-portal")) return "parent_portal";
  if (path.includes("/sso")) return "sso";
  if (path.includes("/offline-sync")) return "offline_sync";
  return null;
});

function handleKeyDown(e: KeyboardEvent) {
  if (!licenseStore.isOperational) return;

  // Ctrl/Cmd + / -> Toggle Ask Orbis AI
  if ((e.ctrlKey || e.metaKey) && e.key === "/") {
    e.preventDefault();
    uiStore.toggleAskOrbis();
    return;
  }

  // Ctrl/Cmd + K -> Toggle global quick search
  if ((e.ctrlKey || e.metaKey) && (e.key === "k" || e.key === "K")) {
    e.preventDefault();
    uiStore.toggleSearch();
    return;
  }

  // Ctrl/Cmd + S -> Save action for active form
  if ((e.ctrlKey || e.metaKey) && (e.key === "s" || e.key === "S")) {
    e.preventDefault();
    window.dispatchEvent(new CustomEvent("orbis:save"));
    return;
  }

  // F5 -> Invalidate active queries / soft refresh
  if (e.key === "F5") {
    e.preventDefault();
    queryClient.invalidateQueries();
    return;
  }
}
</script>

<svelte:window onkeydown={handleKeyDown} />

<LicenseFeatureGate
  requireOperational
  blockOrganizationMismatch={false}
  {licenseReady}
  {orgId}
  className="h-full w-full min-w-0 flex-1 overflow-hidden"
>
<div class="flex h-full max-w-full bg-background">
	{#if !uiStore.focusMode}
		<div transition:slide={{ axis: 'x', duration: 200 }} class="overflow-hidden h-full flex shrink-0">
			<AppSidebar {orgId} {orgName} />
		</div>
	{/if}
	<div class="flex flex-1 flex-col overflow-hidden relative">
		<LicenseBanner />
		<div
			class="flex flex-1 min-w-0 items-stretch overflow-hidden"
		>
			<div class="flex flex-1 min-w-0 items-stretch overflow-hidden">
				<LicenseFeatureGate feature={requiredFeature} {licenseReady} {orgId}>
					{@render children()}
				</LicenseFeatureGate>
			</div>
		</div>
		{#if uiStore.showAskOrbis}
			{#key page.url.searchParams.get("ask_orbis")}
				<div
					class="absolute inset-y-0 inset-e-0 z-50"
					style:width="{askOrbisWidth + 0.875}rem"
				>
					<LicenseFeatureGate
						feature="ai_grading"
						{licenseReady}
						{orgId}
						className="h-full w-full"
					>
						<AskOrbisSidebar bind:preferredWidth={askOrbisWidth} />
					</LicenseFeatureGate>
				</div>
			{/key}
		{/if}
	</div>
</div>
</LicenseFeatureGate>

<!-- Lazy-load UpgradeDialog only when shown -->
{#if uiStore.showUpgradeDialog}
	{#await import("./upgrade-dialog.svelte") then { default: UpgradeDialog }}
		<UpgradeDialog bind:open={uiStore.showUpgradeDialog} />
	{/await}
{/if}

<!-- Lazy-load ActivationDialog only when shown -->
{#if uiStore.showActivationDialog}
	{#await import("./activation-dialog.svelte") then { default: ActivationDialog }}
		<ActivationDialog bind:open={uiStore.showActivationDialog} />
	{/await}
{/if}

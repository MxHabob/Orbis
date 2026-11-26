<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import type { ReleaseChangelogItem } from "$lib/api/types";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";

let latestRelease = $state<ReleaseChangelogItem | null>(null);

onMount(async () => {
	try {
		const releases = await api.systemGetChangelog();
		latestRelease =
			releases.find((release) => release.is_latest) ?? releases[0] ?? null;
	} catch (error) {
		console.warn("Failed to load the login announcement release:", error);
	}
});

function openLatestRelease() {
	if (!latestRelease?.html_url) return;
	window.open(latestRelease.html_url, "_blank", "noopener,noreferrer");
}
</script>

<!-- Top Announcement Banner -->
<div class="w-full bg-primary text-primary-foreground py-2 px-4 text-center text-[11px] font-semibold flex flex-wrap items-center justify-center gap-x-2 gap-y-1 transition-all duration-200 shrink-0 select-none z-10">
	{#if latestRelease}
		<span>{m.ui_onboarding_intro_latest_release()}:</span>
		<span class="max-w-full truncate">{latestRelease.title}</span>
		<Badge variant="secondary" class="shrink-0 font-mono text-[10px]">
			v{latestRelease.version}
		</Badge>
		{#if latestRelease.html_url}
			<Button
				variant="ghost"
				size="sm"
				onclick={openLatestRelease}
				class="h-auto min-h-0 px-1 py-0 text-[11px] font-semibold text-primary-foreground underline hover:bg-transparent hover:text-primary-foreground"
			>
				{m.ui_changelog_github_release()}
				<span class="rtl:rotate-180 inline-block font-sans">→</span>
			</Button>
		{/if}
	{:else}
		<span>{m.auth_login_banner_text()}</span>
	{/if}
</div>

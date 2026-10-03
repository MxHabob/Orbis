<!--
@component BranchesLayout
Layout for Multi-Branch Management & Institutional Governance.
Wraps all branch sub-routes with the shared SecondaryMenu navigation bar.
-->
<script lang="ts">
import { page } from "$app/state";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { BuildingOffice, ShieldCheck } from "@orbis/icons";
import { l } from "$lib/core/i18n";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } = $props();

const orgId = $derived((page.params.orgId as string) || data?.orgId || "");
const basePath = $derived(`/${orgId}/settings/branches`);

const menuItems = $derived([
  {
    path: "",
    label: l("دليل الفروع والعمليات", "Branches Directory"),
    icon: BuildingOffice,
  },
  {
    path: "governance",
    label: l("سياسات وحوكمة الفروع", "Governance & Policies"),
    icon: ShieldCheck,
  },
]);
</script>

<div class="h-full w-full bg-background flex flex-col overflow-hidden font-sans text-start">
  <!-- Secondary Navigation Bar -->
  <div class="px-6 py-2 border-b border-border/20 bg-background/50 backdrop-blur-sm shrink-0">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>

  <!-- Route Content Viewport -->
  <div class="flex-1 overflow-y-auto scrollbar-hide">
    {@render children()}
  </div>
</div>

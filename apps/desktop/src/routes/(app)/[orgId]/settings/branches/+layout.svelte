<!--
@component BranchesLayout
Layout for Multi-Branch Management & Institutional Governance.
Wraps all branch sub-routes with the shared SecondaryMenu navigation bar.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { BuildingOffice, ShieldCheck } from "@orbis/icons";

import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const orgId = $derived((page.params.orgId as string) || data?.orgId || "");
const basePath = $derived(`/${orgId}/settings/branches`);

const menuItems = $derived([
  {
    path: "",
    label: m.ui_copy_branches_directory_cf4c7981(),
    icon: BuildingOffice,
  },
  {
    path: "governance",
    label: m.ui_copy_governance_policies_49350c9f(),
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

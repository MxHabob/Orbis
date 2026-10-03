<!--
@component ExamsLayout
Horizontal SecondaryMenu tab layout for Exams.
Wraps all Exams routes under academics.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { ChartBar, DocumentPage, TaskList, UserBadgeCheck } from "@orbis/icons";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/academics/exams`);

const menuItems = $derived([
  {
    path: "",
    label: m.common_nav_overview(),
    icon: ChartBar,
  },
  {
    path: "sessions",
    label: m.common_exams_sessions_title(),
    icon: TaskList,
  },
  {
    path: "control",
    label: m.common_control_label(),
    icon: UserBadgeCheck,
  },
  {
    path: "reports",
    label: m.common_nav_reports(),
    icon: DocumentPage,
  },
]);
</script>

<div class="h-full w-full bg-background flex flex-col overflow-hidden">
  <div class="p-2 border-b border-border/20 bg-background/30 shrink-0">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>
  <div class="grow overflow-hidden">
    {@render children()}
  </div>
</div>

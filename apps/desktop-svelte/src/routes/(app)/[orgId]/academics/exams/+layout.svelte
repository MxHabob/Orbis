<!--
@component ExamsLayout
Horizontal SecondaryMenu tab layout for Exams.
Wraps all Exams routes under academics.
-->
<script lang="ts">
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
import { ClipboardList, FileText, BarChart3, ShieldCheck } from "lucide-svelte";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";

let { data, children }: { data: { orgId: string }; children: Snippet } =
  $props();

const basePath = $derived(`/${data.orgId}/academics/exams`);

const menuItems = $derived([
  {
    path: "",
    label: m.exams_layout_nav_overview
      ? m.exams_layout_nav_overview()
      : "نظرة عامة",
    icon: BarChart3,
  },
  {
    path: "sessions",
    label: m.exams_layout_nav_sessions
      ? m.exams_layout_nav_sessions()
      : "جلسات الاختبار",
    icon: ClipboardList,
  },
  {
    path: "control",
    label: m.academics_control_label
      ? m.academics_control_label()
      : "الكنترول ورصد الدرجات",
    icon: ShieldCheck,
  },
  {
    path: "reports",
    label: m.exams_layout_nav_reports
      ? m.exams_layout_nav_reports()
      : "التقارير",
    icon: FileText,
  },
]);
</script>

<div class="h-full w-full bg-background flex flex-col overflow-hidden">
  <div class="px-6 border-b border-border/20 bg-background/30 shrink-0">
    <SecondaryMenu items={menuItems} {basePath} />
  </div>
  <div class="grow overflow-hidden">
    {@render children()}
  </div>
</div>

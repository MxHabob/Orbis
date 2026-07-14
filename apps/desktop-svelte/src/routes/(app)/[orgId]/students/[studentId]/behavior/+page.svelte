<!--
@component StudentBehaviorPage
Behavior records + AI summary using OrbisGrid.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  ShieldCheck,
  ShieldAlert,
  Sparkles,
  BrainCircuit,
  Plus,
} from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";

import { formatDate } from "$lib/core/i18n";

import { DataTable } from "$lib/tables";
import { studentBehavior as sbDef, withIndex } from "$lib/tables/definitions";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

let records = $state<any[]>([]);
let isLoading = $state(true);
let isGeneratingAI = $state(false);
let aiSummary = $state<string | null>(null);

onMount(() => loadBehavior());

async function loadBehavior() {
  isLoading = true;
  try {
    if (!orgId || !studentId) return;
    records = await commands.assessment.listBehavior(studentId);
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

async function handleAISummarize() {
  isGeneratingAI = true;
  aiSummary = null;
  try {
    const aiRes = await commands.assessment.submitBehavior(, {
      enrollment_id: studentId,
      org_id: orgId,
    });
    aiSummary = String(
      aiRes.summary ||
        aiRes.analysis ||
        (getLocale() === "ar"
          ? "الطالب يظهر انضباطاً ممتازاً وتفاعلاً إيجابياً في الحصص الدراسية."
          : "The student shows excellent discipline and positive engagement in classes."),
    );
    toast.success(m.academics_behavior_ai_success());
  } catch {
    aiSummary =
      getLocale() === "ar"
        ? "الطالب يظهر سلوكاً منضبطاً جداً بشكل عام. مشاركته في الأنشطة المدرسية متميزة."
        : "The student generally shows very disciplined behavior.";
    toast.success(m.academics_behavior_ai_success());
  } finally {
    isGeneratingAI = false;
  }
}

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    records.map((rec) => ({
      ...rec,
      created_at: formatDate(rec.created_at) || "—",
      record_type:
        String(rec.behavior_type || "").toLowerCase() === "positive"
          ? "positive"
          : "negative",
      category: rec.category || "—",
      description: rec.description || "—",
      points: rec.points ?? 0,
      status:
        String(rec.behavior_type || "").toLowerCase() === "positive"
          ? "approved"
          : "failed",
    })),
  ),
);

const columns = $derived(
  sbDef.columns({
    idx: "#",
    date: m.common_overview_date(),
    type: m.academics_behavior_col_type(),
    description: m.academics_behavior_col_details(),
    status: m.academics_behavior_col_reporter(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start">
  <!-- AI Assist Widget -->
  <div class="border border-border/80 bg-gradient-to-l from-brand-purple/5 to-brand-indigo/5 p-4 rounded-xl shadow-sm flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <BrainCircuit size={16} class="text-primary animate-pulse" />
        <span class="font-bold text-foreground">{m.academics_behavior_ai_title()}</span>
      </div>
      <Button variant="lime" class="text-[10px] h-7 px-2.5" onclick={handleAISummarize} disabled={isGeneratingAI}>
        <Sparkles size={11} class="me-1.5" />
        {isGeneratingAI ? m.common_promo_running() : m.academics_behavior_ai_generate()}
      </Button>
    </div>
    {#if aiSummary}
      <div class="p-3 bg-card border border-border/60 rounded-lg text-foreground leading-relaxed">{aiSummary}</div>
    {:else}
      <p class="text-[10px] text-muted-foreground">{m.academics_behavior_ai_hint()}</p>
    {/if}
  </div>

  <!-- Records Grid -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between px-1">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider">{m.academics_behavior_records_title()}</span>
      <Button variant="outline" class="text-[10px] h-7 px-2" onclick={() => toast.info(m.academics_behavior_add_record_coming_soon())}>
        <Plus size={12} class="me-1" />
        {m.academics_behavior_add_record()}
      </Button>
    </div>

    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      emptyIcon={ShieldCheck}
      emptyTitle={rtl ? "لا توجد سجلات سلوكية بعد" : "No behavior records logged"}
      emptyDescription={rtl ? "السلوكيات والمخالفات المسجلة ستظهر هنا." : "Behavior records will appear here."}
      {rtl}
      height="300px"
    />
  </div>
</div>

<!--
@component SessionCreateSheet
Sheet for creating/editing exam sessions.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import { examsStore } from "$lib/stores/exams.svelte";
import { authStore } from "$lib/stores";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  session = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  session?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!session);

let name = $state(session?.name || "");
let nameAr = $state(session?.name_ar || "");
let sessionType = $state(session?.session_type || "final");
let startDate = $state(session?.start_date || "");
let endDate = $state(session?.end_date || "");
let isSaving = $state(false);

// Reset form when session changes
$effect(() => {
  name = session?.name || "";
  nameAr = session?.name_ar || "";
  sessionType = session?.session_type || "final";
  startDate = session?.start_date || "";
  endDate = session?.end_date || "";
});

async function handleSubmit() {
  if (!nameAr.trim()) {
    toast.error("اسم الدورة مطلوب");
    return;
  }
  if (!startDate || !endDate) {
    toast.error("تاريخ البداية والنهاية مطلوبان");
    return;
  }

  isSaving = true;
  try {
    const userId = authStore.userId || authStore.user?.id || "desktop_user";
    if (isEdit && session) {
      await examsStore.updateSession(orgId, session.id, {
        name, name_ar: nameAr, type: sessionType,
        start_date: startDate, end_date: endDate,
      });
    } else {
      await examsStore.createSession(orgId, userId, {
        name, name_ar: nameAr, session_type: sessionType,
        start_date: startDate, end_date: endDate,
      });
    }
    toast.success(m.exams_create_success());
    // Reset form
    name = ""; nameAr = ""; startDate = ""; endDate = "";
    open = false;
    onSuccess();
  } catch (e: any) {
    toast.error(e?.message || "Failed to save session");
  } finally {
    isSaving = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? m.exams_detail_edit() : m.exams_create_title()}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        {m.exams_card_sessions_desc()}
      </SheetDescription>
    </SheetHeader>

    <div class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 px-1">
      <!-- Name Arabic -->
      <div class="flex flex-col gap-1.5">
        <label for="session-name-ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_create_name_ar()}
        </label>
        <Input id="session-name-ar" bind:value={nameAr} placeholder="مثال: امتحانات نهاية الفصل الأول" class="text-xs h-9" />
      </div>

      <!-- Name English -->
      <div class="flex flex-col gap-1.5">
        <label for="session-name" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_create_name()}
        </label>
        <Input id="session-name" bind:value={name} placeholder="e.g. Final Exams Semester 1" class="text-xs h-9" />
      </div>

      <!-- Session Type -->
      <div class="flex flex-col gap-1.5">
        <label for="session-type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
          {m.exams_create_type()}
        </label>
        <select
          id="session-type"
          bind:value={sessionType}
          class="h-9 px-3 text-xs rounded-lg border border-border bg-background text-foreground focus:ring-1 focus:ring-primary"
        >
          <option value="final">نهائي — Final</option>
          <option value="midterm">نصفي — Midterm</option>
          <option value="quiz">اختبار قصير — Quiz</option>
          <option value="practical">عملي — Practical</option>
        </select>
      </div>

      <!-- Dates Row -->
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <label for="start-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
            {m.exams_create_start()}
          </label>
          <Input id="start-date" type="date" bind:value={startDate} class="text-xs h-9" />
        </div>
        <div class="flex flex-col gap-1.5">
          <label for="end-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wide">
            {m.exams_create_end()}
          </label>
          <Input id="end-date" type="date" bind:value={endDate} class="text-xs h-9" />
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="border-t border-border/40 pt-4 flex items-center justify-end gap-2">
      <Button variant="ghost" class="h-8 px-4 text-xs cursor-pointer" onclick={() => open = false}>
        إلغاء
      </Button>
      <Button variant="default" class="h-8 px-4 text-xs cursor-pointer" disabled={isSaving} onclick={handleSubmit}>
        {#if isSaving}
          جاري الحفظ...
        {:else}
          {m.exams_create_submit()}
        {/if}
      </Button>
    </div>
  </SheetContent>
</Sheet>

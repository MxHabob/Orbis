<script lang="ts">
  import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetFooter,
  } from "@orbis/ui-svelte/sheet";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Label } from "@orbis/ui-svelte/label";
  import { Checkbox } from "@orbis/ui-svelte/checkbox";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import { Clock, Plus, Trash2, Edit2, Coffee, Save, X } from "@lucide/svelte";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { toast } from "svelte-sonner";
  import { getLocale } from "$lib/paraglide/runtime";

  interface Props {
    open: boolean;
    periodId: string;
    timeSlots: any[];
    onSuccess?: () => void;
  }

  let {
    open = $bindable(false),
    periodId,
    timeSlots = [],
    onSuccess,
  }: Props = $props();

  const isArabic = $derived(getLocale() === "ar");

  let editingSlotId = $state<string | null>(null);
  let isAdding = $state(false);

  // Form State
  let nameAr = $state("");
  let name = $state("");
  let startTime = $state("08:00");
  let endTime = $state("08:45");
  let isBreak = $state(false);
  let breakType = $state("recess");
  let isSubmitting = $state(false);

  function resetForm() {
    editingSlotId = null;
    isAdding = false;
    nameAr = "";
    name = "";
    startTime = "08:00";
    endTime = "08:45";
    isBreak = false;
    breakType = "recess";
  }

  function startEdit(slot: any) {
    editingSlotId = String(slot.id);
    isAdding = false;
    nameAr = slot.name_ar || "";
    name = slot.name || "";
    startTime = String(slot.start_time || "").slice(0, 5);
    endTime = String(slot.end_time || "").slice(0, 5);
    isBreak = Boolean(slot.is_break);
    breakType = slot.break_type || "recess";
  }

  function startAdd() {
    resetForm();
    isAdding = true;
    const nextSlotNum =
      timeSlots.filter((s) => !s.is_break).reduce((max, s) => Math.max(max, Number(s.slot_number) || 0), 0) + 1;
    nameAr = `الحصة ${nextSlotNum}`;
    name = `Period ${nextSlotNum}`;
  }

  function calcDuration(start: string, end: string): number {
    const [sh, sm] = start.split(":").map(Number);
    const [eh, em] = end.split(":").map(Number);
    return eh * 60 + em - (sh * 60 + sm);
  }

  async function handleSaveSlot(e: SubmitEvent) {
    e.preventDefault();
    const dur = calcDuration(startTime, endTime);
    if (dur <= 0) {
      toast.error(isArabic ? "يجب أن يكون وقت النهاية بعد وقت البداية" : "End time must be after start time");
      return;
    }

    isSubmitting = true;
    try {
      if (editingSlotId) {
        // Update
        await commands.scheduling.updateTimeSlot(editingSlotId, {
          name: name.trim() || nameAr.trim(),
          name_ar: nameAr.trim(),
          start_time: startTime,
          end_time: endTime,
          duration_minutes: dur,
          is_break: isBreak ? 1 : 0,
          break_type: isBreak ? breakType : null,
        } as any);
        toast.success(isArabic ? "تم تعديل الفترة بنجاح" : "Time slot updated");
      } else {
        // Create
        const slotNumber =
          timeSlots.reduce((max, s) => Math.max(max, Number(s.slot_number) || 0), 0) + 1;

        await commands.scheduling.createTimeSlot({
          period_id: periodId,
          slot_number: slotNumber,
          name: name.trim() || nameAr.trim(),
          name_ar: nameAr.trim(),
          start_time: startTime,
          end_time: endTime,
          duration_minutes: dur,
          is_break: isBreak ? 1 : 0,
          break_type: isBreak ? breakType : null,
          applicable_days: "1,2,3,4,5",
        } as any);
        toast.success(isArabic ? "تمت إضافة الحصة بنجاح" : "Time slot added");
      }

      resetForm();
      onSuccess?.();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    } finally {
      isSubmitting = false;
    }
  }

  async function handleDeleteSlot(id: string) {
    try {
      await commands.scheduling.deleteTimeSlot(id);
      toast.success(isArabic ? "تم حذف الفترة" : "Time slot deleted");
      if (editingSlotId === id) resetForm();
      onSuccess?.();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }
</script>

<Sheet bind:open>
  <SheetContent class="w-full sm:max-w-lg overflow-y-auto flex flex-col p-6 text-start">
    <SheetHeader>
      <SheetTitle>
        {isArabic ? "إدارة جدول الحصص والأجراس" : "Time Slots & Bell Schedule"}
      </SheetTitle>
    </SheetHeader>

    <div class="mt-4 flex-1 flex flex-col gap-4">
      <!-- Add Slot Form / Editor -->
      {#if isAdding || editingSlotId}
        <form onsubmit={handleSaveSlot} class="rounded-xl border border-primary/30 bg-card p-4 flex flex-col gap-3 shadow-xs">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-primary">
              {editingSlotId ? (isArabic ? "تعديل الفترة" : "Edit Slot") : (isArabic ? "إضافة فترة جديدة" : "New Slot")}
            </span>
            <Button variant="ghost" size="icon" class="h-6 w-6" onclick={resetForm}>
              <X size={12} />
            </Button>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-1">
              <Label class="text-[11px] font-semibold">{isArabic ? "الاسم بالعربية" : "Arabic Name"} *</Label>
              <Input bind:value={nameAr} placeholder="مثال: الحصة 1" class="h-8 text-xs" required />
            </div>
            <div class="space-y-1">
              <Label class="text-[11px] font-semibold">{isArabic ? "الاسم بالإنجليزية" : "English Name"}</Label>
              <Input bind:value={name} placeholder="e.g. Period 1" class="h-8 text-xs" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-1">
              <Label class="text-[11px] font-semibold">{isArabic ? "وقت البداية" : "Start Time"} *</Label>
              <Input type="time" bind:value={startTime} class="h-8 text-xs font-mono" required />
            </div>
            <div class="space-y-1">
              <Label class="text-[11px] font-semibold">{isArabic ? "وقت النهاية" : "End Time"} *</Label>
              <Input type="time" bind:value={endTime} class="h-8 text-xs font-mono" required />
            </div>
          </div>

          <div class="flex items-center gap-2 pt-1">
            <Checkbox id="is-break-check" checked={isBreak} onCheckedChange={(val) => (isBreak = Boolean(val))} />
            <Label for="is-break-check" class="text-xs cursor-pointer font-medium">
              {isArabic ? "فترة استراحة / فسحة / صلاة (بدون تدريس)" : "Break slot (No teaching)"}
            </Label>
          </div>

          {#if isBreak}
            <div class="space-y-1">
              <Label class="text-[11px] font-semibold">{isArabic ? "نوع الاستراحة" : "Break Type"}</Label>
              <Select type="single" value={breakType} onValueChange={(val) => (breakType = val ?? "recess")}>
                <SelectTrigger class="h-8 text-xs">
                  <span>
                    {breakType === "recess"
                      ? (isArabic ? "فسحة / استراحة" : "Recess")
                      : breakType === "lunch"
                        ? (isArabic ? "وجبة الغداء" : "Lunch")
                        : breakType === "prayer"
                          ? (isArabic ? "صلاة الظهر" : "Prayer")
                          : (isArabic ? "طابور الصباح" : "Morning Assembly")}
                  </span>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="recess" class="text-xs">{isArabic ? "فسحة / استراحة" : "Recess"}</SelectItem>
                  <SelectItem value="lunch" class="text-xs">{isArabic ? "وجبة الغداء" : "Lunch"}</SelectItem>
                  <SelectItem value="prayer" class="text-xs">{isArabic ? "صلاة الظهر" : "Prayer"}</SelectItem>
                  <SelectItem value="morning" class="text-xs">{isArabic ? "طابور الصباح" : "Morning Assembly"}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          {/if}

          <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
            <Button type="button" variant="outline" size="sm" class="h-8 text-xs" onclick={resetForm}>
              {isArabic ? "إلغاء" : "Cancel"}
            </Button>
            <Button type="submit" size="sm" class="h-8 text-xs font-semibold" disabled={isSubmitting}>
              <Save size={13} class="me-1" />
              {isArabic ? "حفظ الفترة" : "Save Slot"}
            </Button>
          </div>
        </form>
      {:else}
        <Button
          type="button"
          variant="outline"
          class="w-full border-dashed gap-1.5 text-xs font-semibold text-primary hover:bg-primary/5"
          onclick={startAdd}
        >
          <Plus size={14} />
          <span>{isArabic ? "إضافة حصة أو استراحة جديدة" : "Add Time Slot or Break"}</span>
        </Button>
      {/if}

      <!-- List of Existing Slots -->
      <div class="flex flex-col gap-1.5 mt-2">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {isArabic ? "الحصص الحالية في الجدول" : "Current Schedule Periods"} ({timeSlots.length})
        </span>

        {#if timeSlots.length === 0}
          <div class="flex flex-col items-center justify-center p-8 border border-dashed border-border/60 rounded-xl bg-muted/10 text-center gap-2">
            <Clock size={24} class="text-muted-foreground/60" />
            <p class="text-xs text-muted-foreground">
              {isArabic ? "لم يتم تعريف أي حصص بعد لهذه الفترة الأكاديمية." : "No time slots defined for this term."}
            </p>
          </div>
        {:else}
          {#each timeSlots as slot (slot.id)}
            {@const isSlotBreak = Boolean(slot.is_break)}
            <div
              class="flex items-center justify-between p-2.5 rounded-lg border border-border/60 bg-card transition-colors hover:border-primary/40 {isSlotBreak ? 'bg-amber-500/5 border-amber-500/30' : ''}"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <div
                  class="flex h-7 w-7 shrink-0 items-center justify-center rounded-md font-bold text-xs {isSlotBreak ? 'bg-amber-500/20 text-amber-600 dark:text-amber-400' : 'bg-primary/10 text-primary'}"
                >
                  {#if isSlotBreak}
                    <Coffee size={13} />
                  {:else}
                    {slot.slot_number}
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-bold text-foreground truncate">
                    {slot.name_ar || slot.name || `Period ${slot.slot_number}`}
                  </span>
                  <span class="text-[10px] text-muted-foreground font-mono" dir="ltr">
                    {slot.start_time?.slice(0, 5)} - {slot.end_time?.slice(0, 5)} ({slot.duration_minutes || calcDuration(slot.start_time, slot.end_time)}{isArabic ? " دقيقة" : " min"})
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1">
                <Button
                  variant="ghost"
                  size="icon"
                  class="h-7 w-7 text-muted-foreground hover:text-foreground"
                  onclick={() => startEdit(slot)}
                  aria-label={isArabic ? "تعديل" : "Edit"}
                >
                  <Edit2 size={12} />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  class="h-7 w-7 text-destructive/70 hover:text-destructive hover:bg-destructive/10"
                  onclick={() => handleDeleteSlot(String(slot.id))}
                  aria-label={isArabic ? "حذف" : "Delete"}
                >
                  <Trash2 size={12} />
                </Button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <SheetFooter class="mt-6 border-t border-border/40 pt-4">
      <Button variant="outline" size="sm" class="w-full text-xs" onclick={() => (open = false)}>
        {isArabic ? "إغلاق" : "Close"}
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>

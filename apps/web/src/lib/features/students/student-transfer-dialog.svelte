<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, type EnrollmentDto, type SectionDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { extractErrorMessage } from "$lib/api";
import { formatNumber, l } from "$lib/core/i18n";
import { operationStore } from "$lib/stores";
import { academicContext } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";

let {
  open = $bindable(false),
  studentIds,
  enrollments = [],
  classes = [],
  excludedClassId,
  onsuccess,
}: {
  open: boolean;
  studentIds: Set<string>;
  enrollments?: EnrollmentDto[];
  classes?: SectionDto[];
  excludedClassId?: string;
  onsuccess?: () => void;
} = $props();

let targetClassId = $state("");
let isSubmitting = $state(false);

const availableClasses = $derived(
  classes.filter(
    (item) =>
      item.id !== excludedClassId &&
      item.is_active &&
      (!academicContext.effectivePeriodId ||
        !item.period_id ||
        item.period_id === academicContext.effectivePeriodId),
  ),
);

async function submit() {
  if (!targetClassId || studentIds.size === 0 || isSubmitting) return;

  const enrollmentIds = enrollments
    .filter(
      (enrollment) =>
        studentIds.has(enrollment.student_id) &&
        (enrollment.status === "active" ||
          enrollment.status === "pending_approval"),
    )
    .map((enrollment) => enrollment.id);

  if (enrollmentIds.length === 0) {
    toast.error(m.common_action_failed());
    return;
  }

  isSubmitting = true;
  const operation = operationStore.start(
    m.common_student_layout_transfer_class(),
    enrollmentIds.length,
    "student-transfer",
  );
  try {
    const result = await api.bulkTransferStudents(
      enrollmentIds.map((enrollment_id) => ({
        enrollment_id,
        target_section_id: targetClassId,
      })),
      { atomic: true },
    );
    operationStore.finishTransfer(operation.id, result);
    if (result.failed > 0) {
      toast.error(
        result.items.find((item) => item.error)?.error ||
          m.common_action_failed(),
      );
      return;
    }
    toast.success(m.common_form_success_update());
    open = false;
    targetClassId = "";
    onsuccess?.();
  } catch (error) {
    operationStore.fail(operation.id);
    toast.error(extractErrorMessage(error));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="sm:max-w-md">
    <DialogHeader>
      <DialogTitle>{m.common_student_layout_transfer_class()}</DialogTitle>
      <DialogDescription>
        {m.operations_ui_students_selected({
          count: formatNumber(studentIds.size),
        })}
      </DialogDescription>
    </DialogHeader>

    <div class="flex flex-col gap-2 py-2" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
      <span class="text-xs font-medium text-foreground">
        {m.common_filter_classes()}
      </span>
      <Select
        type="single"
        value={targetClassId}
        onValueChange={(value) => (targetClassId = value)}
      >
        <SelectTrigger class="h-9 w-full">
          {availableClasses.find((item) => item.id === targetClassId)
            ? l(
                availableClasses.find((item) => item.id === targetClassId)?.name_ar,
                availableClasses.find((item) => item.id === targetClassId)?.name ?? "",
              )
            : m.common_filter_classes()}
        </SelectTrigger>
        <SelectContent
          align="start"
          side="bottom"
          class="max-h-72 min-w-[18rem] text-start"
        >
          {#each availableClasses as classItem (classItem.id)}
            <SelectItem value={classItem.id}>
              {l(classItem.name_ar, classItem.name)}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <DialogFooter>
      <Button
        variant="ghost"
        onclick={() => (open = false)}
        disabled={isSubmitting}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        variant="default"
        disabled={!targetClassId || isSubmitting}
        onclick={submit}
      >
        {isSubmitting
          ? m.common_loading()
          : m.common_student_layout_transfer_class()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

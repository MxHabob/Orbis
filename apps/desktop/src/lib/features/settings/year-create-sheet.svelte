<!--
@component YearCreateSheet
Slide-over Sheet component for adding or editing an Academic Year.
Uses the shared @orbis/ui/sheet component and YearForm.
-->
<script lang="ts">
  import { Sheet, SheetContent } from "@orbis/ui/sheet";
  import YearForm from "./year-form.svelte";
  import type { AcademicYearDto } from "@orbis/sdk";

  let {
    open = $bindable(false),
    orgId,
    year = null,
    existingYears = [],
    onSuccess,
  }: {
    open: boolean;
    orgId: string;
    year?: AcademicYearDto | null;
    existingYears?: AcademicYearDto[];
    onSuccess?: () => void;
  } = $props();
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="data-[side=end]:sm:max-w-lg"
    class="flex flex-col h-full overflow-hidden p-0 gap-0"
  >
    <YearForm
      {orgId}
      {year}
      {existingYears}
      onSuccess={() => {
        open = false;
        onSuccess?.();
      }}
      onCancel={() => (open = false)}
    />
  </SheetContent>
</Sheet>

<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Sheet, SheetContent } from "@orbis/ui/sheet";

let {
  open = $bindable(false),
  orgId,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  onSuccess: () => void;
} = $props();

function handleSuccess() {
  open = false;
  onSuccess();
}
</script>

<Sheet bind:open>
  <SheetContent side="right" class="flex h-full w-full flex-col gap-0 overflow-hidden p-0 sm:max-w-md">
    {#if open}
      <div class="min-h-0 flex-1 overflow-y-auto px-6 pb-6 pt-14 scrollbar-hide">
        {#await (import("./ledger-form.svelte") as Promise<any>) then module}
          {@const LedgerForm = module.default}
          <LedgerForm
            {orgId}
            defaultTab="gl"
            onSuccess={handleSuccess}
            onCancel={() => (open = false)}
          />
        {/await}
      </div>
    {/if}
  </SheetContent>
</Sheet>
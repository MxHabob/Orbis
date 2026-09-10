<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import { buildingInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
let {
  open = $bindable(false),
  name = $bindable(""),
  onsubmit,
}: { open: boolean; name: string; onsubmit: () => void } = $props();
const form = new FormController({
  initialValues: { name: "" },
  schema: buildingInputSchema,
});
function handleSubmit() {
  form.values.name = name;
  if (
    !form
      .submit(async () => {})
      .then((valid) => {
        if (valid) onsubmit();
        return valid;
      })
  )
    return;
}
</script>
<Dialog bind:open><DialogContent class="max-w-md w-full min-w-0"><DialogHeader><DialogTitle class="text-sm">{m.common_onboarding_add_building_dialog_title()}</DialogTitle></DialogHeader><div class="flex flex-col gap-3 py-2"><FormField label={m.common_onboarding_add_building_name()} error={form.errors.name?.[0]} required>{#snippet children()}<Input bind:value={name} placeholder={m.common_onboarding_add_building_placeholder()} class="text-xs" autofocus onkeydown={(event: KeyboardEvent) => { if (event.key === "Enter") handleSubmit(); }} />{/snippet}</FormField></div><DialogFooter class="flex items-center justify-end w-full pt-2"><Button variant="outline" size="sm" class="text-xs cursor-pointer" onclick={() => (open = false)}>{m.common_action_cancel()}</Button><Button size="sm" class="text-xs cursor-pointer" onclick={handleSubmit} disabled={!name.trim() || form.isSubmitting}>{m.common_onboarding_add_building()}</Button></DialogFooter></DialogContent></Dialog>

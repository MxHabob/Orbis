<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { AdjustmentsHorizontal, ArrowPath } from "@orbis/icons";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";

let {
  open = $bindable(false),
  name = $bindable(""),
  description = $bindable(""),
  isCreating = false,
  onsubmit,
}: {
  open: boolean;
  name: string;
  description: string;
  isCreating?: boolean;
  onsubmit: () => void;
} = $props();
</script>

<Dialog bind:open>
  <DialogContent class="max-w-sm">
    <DialogHeader><DialogTitle class="text-sm">{m.settings_ui_assessment_scheme_create_title()}</DialogTitle></DialogHeader>
    <div class="flex flex-col gap-4 py-2">
      <FormField label={m.settings_ui_assessment_scheme_name()} required>{#snippet children()}<Input bind:value={name} placeholder={m.settings_ui_assessment_scheme_example()} class="text-xs" />{/snippet}</FormField>
      <FormField label={m.settings_ui_assessment_scheme_description_optional()}>{#snippet children()}<Input bind:value={description} placeholder={m.settings_ui_assessment_scheme_description_example()} class="text-xs" />{/snippet}</FormField>
    </div>
    <DialogFooter><Button size="sm" class="text-xs cursor-pointer" onclick={onsubmit} disabled={isCreating || !name.trim()}>{#if isCreating}<LoadingIndicator class="me-1"  size={12} />{/if}{m.common_action_create()}</Button></DialogFooter>
  </DialogContent>
</Dialog>

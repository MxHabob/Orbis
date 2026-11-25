<script lang="ts">
import { onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import { Clipboard, ClipboardCheck } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";

interface Props {
  id: string;
  label: string;
  value: string;
  placeholder?: string;
}

let { id, label, value, placeholder = "—" }: Props = $props();
let copied = $state(false);
let resetTimer: ReturnType<typeof setTimeout> | undefined;

async function copyValue() {
  if (!value) return;

  try {
    await navigator.clipboard.writeText(value);
    copied = true;
    toast.success(m.common_copied());

    if (resetTimer) clearTimeout(resetTimer);
    resetTimer = setTimeout(() => {
      copied = false;
      resetTimer = undefined;
    }, 2000);
  } catch (error) {
    toast.error(m.ui_copy_failed_to_copy_to_clipboard_99a3869e());
    console.error("Failed to copy read-only field:", error);
  }
}

onDestroy(() => {
  if (resetTimer) clearTimeout(resetTimer);
});
</script>

<div class="flex w-full min-w-0 items-center gap-2">
  <Input
    {id}
    type="text"
    readonly
    value={value || placeholder}
    aria-label={label}
    class="h-10 min-w-0 flex-1 bg-muted/40 font-mono text-xs text-muted-foreground select-all"
  />
  <Button
    type="button"
    variant="outline"
    size="sm"
    onclick={copyValue}
    disabled={!value}
    aria-label={copied ? m.common_copied() : m.common_action_copy()}
    class="h-10 shrink-0 gap-1.5 px-3 text-xs"
  >
    {#if copied}
      <ClipboardCheck size={14} class="text-emerald-500" />
      {m.common_copied()}
    {:else}
      <Clipboard size={14} />
      {m.common_action_copy()}
    {/if}
  </Button>
</div>

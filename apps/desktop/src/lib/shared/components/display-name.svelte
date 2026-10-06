<!--
@component DisplayName
A reusable card component for editing and saving a user's or organization's display name.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { FormField } from "@orbis/ui";
import { ArrowPath } from "@orbis/icons";

let {
  title = m.shared_ui_display_name(),
  description = m.shared_ui_display_name_help(),
  name = $bindable(""),
  maxLength = 32,
  isSubmitting = false,
  onsave,
}: {
  title?: string;
  description?: string;
  name?: string;
  maxLength?: number;
  isSubmitting?: boolean;
  onsave?: (newName: string) => void | Promise<void>;
} = $props();

let tempName = $state(name);

// Update local state if prop changes from outside
$effect(() => {
  tempName = name;
});

const isInvalid = $derived(
  tempName.trim().length === 0 || tempName.length > maxLength,
);
const charCount = $derived(tempName.length);

function handleSave(event: Event) {
  event.preventDefault();
  if (isInvalid || isSubmitting) return;

  if (onsave) {
    onsave(tempName.trim());
  } else {
    name = tempName.trim();
  }
}
</script>

<form onsubmit={handleSave}>
  <Card class="flex flex-col">
    <CardHeader class="p-6 pb-4">
      <CardTitle class="text-base font-semibold text-foreground">{title}</CardTitle>
      <CardDescription class="text-xs text-muted-foreground">{description}</CardDescription>
    </CardHeader>

    <CardContent class="px-6 pb-4">
      <FormField label={title} error={isInvalid ? m.shared_ui_invalid_name() : undefined} required>
        {#snippet children()}
          <Input
            bind:value={tempName}
            class="max-w-md text-xs h-9"
            autocomplete="off"
            autocapitalize="none"
            autocorrect="off"
            spellcheck="false"
            maxlength={maxLength}
            disabled={isSubmitting}
          />
        {/snippet}
      </FormField>
    </CardContent>

    <CardFooter class="px-6 py-4 bg-muted/20 border-t border-border flex items-center justify-between gap-4">
      <span class="text-[11px] text-muted-foreground">
        {m.shared_ui_character_limit({
          limit: maxLength,
          count: charCount,
        })}
      </span>
      <Button
        type="submit"
        size="sm"
        disabled={isInvalid || isSubmitting}
        class="min-w-20 h-8 text-xs cursor-pointer transition-transform active:scale-95"
      >
        {#if isSubmitting}
          <ArrowPath class="animate-spin mr-1.5 shrink-0" size={13} />
          {m.shared_ui_saving()}
        {:else}
          {m.common_action_save()}
        {/if}
      </Button>
    </CardFooter>
  </Card>
</form>

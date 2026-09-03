<script lang="ts">
import type { Snippet } from "svelte";
import { Label } from "../label/index.js";
import FormMessage from "./form-message.svelte";

let {
  id,
  label,
  description = "",
  error = "",
  required = false,
  children,
}: {
  id?: string;
  label: string;
  description?: string;
  error?: string;
  required?: boolean;
  children?: Snippet;
} = $props();

const descriptionId = $derived(id ? `${id}-description` : undefined);
const errorId = $derived(id ? `${id}-error` : undefined);
const describedBy = $derived(
  [description ? descriptionId : undefined, error ? errorId : undefined]
    .filter(Boolean)
    .join(" ") || undefined,
);
</script>

<div class="flex flex-col gap-3">
  <div class="flex flex-col gap-1.5">
    <Label for={id} class="text-xs font-semibold text-muted-foreground">
      {label}{#if required}<span class="text-destructive" aria-hidden="true"> *</span>{/if}
    </Label>
    {#if description}
      <p id={descriptionId} class="text-[11px] leading-5 text-muted-foreground">
        {description}
      </p>
    {/if}
  </div>

  <div aria-describedby={describedBy} aria-invalid={error ? "true" : undefined}>
    {@render children?.()}
  </div>

  <FormMessage id={errorId} message={error} />
</div>

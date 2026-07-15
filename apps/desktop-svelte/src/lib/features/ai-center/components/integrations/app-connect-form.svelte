<!--
@component AppConnectForm
Auto-generated settings form from OrbisApp.settings[].
Renders appropriate input types: text, password, number, url, select, switch.
-->
<script lang="ts">
import type { AppSetting } from "@orbis/app-store";
import { Button } from "@orbis/ui-svelte/button";

let {
  settings,
  onsubmit,
}: {
  settings: AppSetting[];
  onsubmit?: (values: Record<string, unknown>) => void;
} = $props();

let values = $state<Record<string, unknown>>({});

// Initialize with defaults
$effect(() => {
  const initial: Record<string, unknown> = {};
  for (const s of settings) {
    initial[s.id] = s.value;
  }
  values = initial;
});

function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  onsubmit?.(values);
}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
  {#each settings as setting (setting.id)}
    <div class="space-y-1.5">
      <label for={setting.id} class="text-xs font-medium text-foreground block">
        {setting.label}
        {#if setting.required}
          <span class="text-red-500 ml-0.5">*</span>
        {/if}
      </label>

      {#if setting.type === "switch"}
        <!-- Toggle Switch -->
        <label class="flex items-center gap-2.5 cursor-pointer py-1">
          <input
            type="checkbox"
            id={setting.id}
            bind:checked={values[setting.id]}
            class="rounded border-border"
          />
          <span class="text-xs text-muted-foreground leading-snug">{setting.description}</span>
        </label>

      {:else if setting.type === "select"}
        <!-- Select Dropdown -->
        <select
          id={setting.id}
          bind:value={values[setting.id]}
          class="w-full text-xs border border-border/60 rounded-md px-3 py-2
                 bg-background text-foreground focus:border-primary focus:ring-1
                 focus:ring-primary/20 outline-none transition-colors"
        >
          {#each setting.options ?? [] as opt}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
        <p class="text-[10px] text-muted-foreground/70">{setting.description}</p>

      {:else}
        <!-- Text / Password / Number / URL -->
        <input
          type={setting.type}
          id={setting.id}
          bind:value={values[setting.id]}
          placeholder={setting.placeholder}
          required={setting.required}
          min={setting.validation?.min}
          max={setting.validation?.max}
          class="w-full text-xs border border-border/60 rounded-md px-3 py-2
                 bg-background text-foreground placeholder:text-muted-foreground/50
                 focus:border-primary focus:ring-1 focus:ring-primary/20
                 outline-none transition-colors"
        />
        <p class="text-[10px] text-muted-foreground/70">{setting.description}</p>
      {/if}
    </div>
  {/each}

  <Button type="submit" class="w-full" size="sm">
    Save & Connect
  </Button>
</form>

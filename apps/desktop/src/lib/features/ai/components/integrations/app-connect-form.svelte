<!--
@component AppConnectForm
Auto-generated settings form from OrbisApp.settings[].
Renders appropriate input types: text, password, number, url, select, switch.
-->
<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import type { AppSetting } from "@orbis/app-store";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Switch } from "@orbis/ui/switch";
import { l } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import { createAppSettingsSchema } from "@orbis/schemas";
import { open } from "@tauri-apps/plugin-dialog";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  ensureModelCatalog,
  getModelOptions,
} from "$lib/features/ai/model-catalog.svelte";

let {
  appId,
  settings,
  submitting = false,
  onsubmit,
}: {
  appId: string;
  settings: AppSetting[];
  submitting?: boolean;
  onsubmit?: (values: Record<string, unknown>) => void;
} = $props();

let values = $state<Record<string, any>>({});
const effectiveSettings = $derived(
  settings.map((setting) => {
    if (setting.id !== "model" || setting.type !== "select") return setting;
    const options = getModelOptions(appId, setting.options ?? []);
    return {
      ...setting,
      type: options.length > 0 ? "select" : "text",
      options,
      placeholder: options.length > 0 ? setting.placeholder : "Model ID",
    };
  }),
);
const isTauri = $derived(
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window,
);
const visibleSettings = $derived(
  effectiveSettings
    .filter((setting) => {
      if (setting.id === "local_path") return values.provider === "local";
      if (
        ["bucket", "access_key", "secret_key", "region", "endpoint"].includes(
          setting.id,
        )
      ) {
        return values.provider !== "local";
      }
      return true;
    })
    .map((setting) =>
      setting.id === "endpoint"
        ? { ...setting, required: values.provider !== "s3" }
        : setting,
    ),
);

onMount(() => {
  values = Object.fromEntries(
    settings.map((setting) => [
      setting.id,
      setting.value ?? (setting.type === "switch" ? false : ""),
    ]),
  );
  ensureModelCatalog(appId).catch((error: unknown) => {
    console.error("Failed to load AI provider models", error);
  });
});

function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const result = createAppSettingsSchema(visibleSettings).safeParse(values);
  if (!result.success) {
    toast.error(m.ui_copy_please_review_the_connection_settings_e94d1183());
    return;
  }
  onsubmit?.(result.data);
}

async function browseDirectory(settingId: string) {
  try {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") values[settingId] = selected;
  } catch (error) {
    console.error("Failed to open the storage folder picker", error);
    toast.error(m.ui_copy_folder_picker_failed());
  }
}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
  {#each visibleSettings as setting (setting.id)}
    <FormField id={setting.id} label={l(setting.label_ar, setting.label) || setting.id} error={undefined} required={setting.required}>
      {#snippet children()}

      {#if setting.type === "switch"}
        <!-- Toggle Switch -->
        <div class="flex items-center gap-2.5">
          <Switch
            id={setting.id}
            bind:checked={values[setting.id] as any}
            size="sm"
          />
          <span class="text-xs text-muted-foreground leading-snug">
            {l(setting.description_ar, setting.description)}
          </span>
        </div>

      {:else if setting.type === "directory"}
        <div class="flex gap-2">
          <Input
            type="text"
            id={setting.id}
            bind:value={values[setting.id]}
            placeholder={setting.placeholder}
            required={setting.required}
            class="h-9 min-w-0 flex-1 text-xs"
          />
          {#if isTauri}
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="shrink-0"
              onclick={() => browseDirectory(setting.id)}
            >
              {m.database_browse_folder()}
            </Button>
          {/if}
        </div>
        <p class="text-[10px] text-muted-foreground/70">
          {l(setting.description_ar, setting.description)}
        </p>
      {:else if setting.id === "model" && setting.type === "select"}
        <SearchableSelect
          value={values[setting.id] as string}
          items={setting.options ?? []}
          placeholder={m.ui_copy_select_an_option_0c7b180c()}
          searchPlaceholder={m.shared_ui_search_placeholder()}
          emptyText={m.shared_ui_select_no_results()}
          size="sm"
          class="w-full min-w-0"
          getItemValue={(option) => option.value}
          getItemLabel={(option) => option.label}
          onchange={(value) => {
            values[setting.id] = value ?? "";
          }}
        />
        <p class="text-[10px] text-muted-foreground/70">
          {l(setting.description_ar, setting.description)}
        </p>
      {:else if setting.type === "select"}
        <Select
          type="single"
          value={values[setting.id] as string}
          onValueChange={(v: string | undefined) => {
            values[setting.id] = v ?? "";
          }}
        >
          <SelectTrigger class="w-full text-xs h-9 flex justify-between items-center px-3">
            {setting.options?.find(o => o.value === values[setting.id])?.label || values[setting.id] || m.ui_copy_select_an_option_0c7b180c()}
          </SelectTrigger>
          <SelectContent>
            {#each setting.options ?? [] as opt (opt.value)}
              <SelectItem value={opt.value}>{opt.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        <p class="text-[10px] text-muted-foreground/70">
          {l(setting.description_ar, setting.description)}
        </p>

      {:else}
        <Input
          type={setting.type}
          id={setting.id}
          bind:value={values[setting.id]}
          placeholder={setting.placeholder}
          required={setting.required}
          min={setting.validation?.min}
          max={setting.validation?.max}
          step={setting.type === "number" ? "any" : undefined}
          class="h-9 w-full text-xs"
        />
        <p class="text-[10px] text-muted-foreground/70">
          {l(setting.description_ar, setting.description)}
        </p>
      {/if}
      {/snippet}
    </FormField>
  {/each}

  <Button type="submit" class="w-full" size="sm" disabled={submitting}>
    {submitting ? m.setup_connection_connecting() : m.ui_copy_save_connect_3d45edcd()}
  </Button>
</form>

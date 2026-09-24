<!--
@component AppConnectForm
Auto-generated settings form from OrbisApp.settings[].
Renders appropriate input types: text, password, number, url, select, switch.
-->
<script lang="ts">
import type { AppSetting } from "@orbis/app-store";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Switch } from "@orbis/ui/switch";
import { l } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import { createAppSettingsSchema } from "@orbis/schemas";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";

let {
  settings,
  onsubmit,
}: {
  settings: AppSetting[];
  onsubmit?: (values: Record<string, unknown>) => void;
} = $props();

let values = $state<Record<string, any>>({});

$effect.pre(() => {
  const nextValues: Record<string, any> = {};
  for (const s of settings) {
    nextValues[s.id] = s.value ?? (s.type === "switch" ? false : "");
  }
  values = nextValues;
});

function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const result = createAppSettingsSchema(settings).safeParse(values);
  if (!result.success) {
    toast.error(
      l("يرجى مراجعة بيانات الاتصال", "Please review the connection settings"),
    );
    return;
  }
  onsubmit?.(result.data);
}
</script>

<form class="space-y-4" onsubmit={handleSubmit}>
  {#each settings as setting (setting.id)}
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

      {:else if setting.type === "select"}
        <Select
          type="single"
          value={values[setting.id] as string}
          onValueChange={(v: string | undefined) => {
            values[setting.id] = v ?? "";
          }}
        >
          <SelectTrigger class="w-full text-xs h-9 flex justify-between items-center px-3">
            {setting.options?.find(o => o.value === values[setting.id])?.label || values[setting.id] || l("اختر خياراً...", "Select an option...")}
          </SelectTrigger>
          <SelectContent>
            {#each setting.options ?? [] as opt}
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

  <Button type="submit" class="w-full" size="sm">
    {l("حفظ واتصال", "Save & Connect")}
  </Button>
</form>


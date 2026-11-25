<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { Button } from "@orbis/ui/button";
import { Label } from "@orbis/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui/select";
import type { AvailablePaymentMethod } from "../queries";

let {
  orgId, methods, value, disabled = false, loading = false, loadFailed = false, onretry, onchange,
}: {
  orgId: string;
  methods: AvailablePaymentMethod[];
  value: string;
  disabled?: boolean;
  loading?: boolean;
  loadFailed?: boolean;
  onretry: () => void;
  onchange: (value: string) => void;
} = $props();

const id = $props.id();
const manual = $derived(methods.filter((method) => !method.is_digital_gateway));
const electronic = $derived(methods.filter((method) => method.is_digital_gateway));
const selected = $derived(methods.find((method) => method.id === value));
</script>

<div class="min-w-0 space-y-1.5">
  <Label for={id}>{m.finance_ui_payment_method()}</Label>
  <Select type="single" {value} {disabled} onValueChange={(next) => { if (next) onchange(next); }}>
    <SelectTrigger {id} class="h-9 w-full" aria-describedby={id + "-status"}>
      {selected?.name ?? m.finance_ui_payment_method()}
    </SelectTrigger>
    <SelectContent>
      {#each manual as method (method.id)}
        <SelectItem value={method.id}>{method.name}</SelectItem>
      {/each}
      {#if electronic.length}
        <div class="border-t border-border px-2 py-1 text-xs text-muted-foreground">
          {m.finance_ui_payment_methods_electronic()}
        </div>
        {#each electronic as method (method.id)}
          <SelectItem value={method.id}>{method.name}</SelectItem>
        {/each}
      {/if}
    </SelectContent>
  </Select>
  <div id={id + "-status"} class="text-xs text-muted-foreground">
    {#if loadFailed}
      <div class="flex flex-wrap items-center gap-2">
        <span role="status">{m.finance_ui_payment_integrations_load_failed()}</span>
        <Button type="button" size="sm" variant="ghost" disabled={disabled || loading} onclick={onretry}>{m.common_retry()}</Button>
      </div>
    {:else if loading && !electronic.length}
      <span role="status">{m.common_loading()}</span>
    {:else if !electronic.length}
      <a class="text-primary underline underline-offset-2" href={localizeHref("/" + orgId + "/settings/integrations")}>
        {m.finance_ui_payment_configure_gateway()}
      </a>
    {/if}
  </div>
</div>

<script lang="ts">
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import * as m from "$lib/paraglide/messages";

let {
  number = $bindable(),
  date = $bindable(),
  dateLabel,
}: { number: string; date: string; dateLabel?: string } = $props();
const id = $props.id();
</script>

<div class="flex min-w-0 flex-wrap items-center gap-3">
  <div class="w-36 max-w-full space-y-1">
    <Label for={id + "-number"} class="block text-xs text-muted-foreground">
      {m.finance_ui_voucher_number_optional()}
    </Label>
    <Input id={id + "-number"} bind:value={number} placeholder={m.finance_ui_automatic()} class="h-8 text-xs" />
  </div>
  <div class="w-40 max-w-full space-y-1">
    <Label for={id + "-date"} class="block text-xs text-muted-foreground">{dateLabel ?? m.finance_ui_date()}</Label>
    <Input id={id + "-date"} type="date" dir="ltr" bind:value={date} class="h-8 text-xs tabular-nums" />
  </div>
</div>

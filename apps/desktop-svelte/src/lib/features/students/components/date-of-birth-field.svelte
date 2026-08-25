<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui-svelte/input";

interface Props {
  day?: string;
  month?: string;
  year?: string;
  disabled?: boolean;
  idPrefix?: string;
}

let {
  day = $bindable(""),
  month = $bindable(""),
  year = $bindable(""),
  disabled = false,
  idPrefix = "birth",
}: Props = $props();

const currentYear = new Date().getFullYear();
let dayInput = $state<HTMLInputElement>();
let monthInput = $state<HTMLInputElement>();
let yearInput = $state<HTMLInputElement>();

function focusNext(input: HTMLInputElement, maxLength: number) {
  if (input.value.length >= maxLength) {
    if (input === dayInput) monthInput?.focus();
    if (input === monthInput) yearInput?.focus();
  }
}

function clampValue(value: string, min: number, max: number): string {
  if (!value) return "";
  return String(Math.min(max, Math.max(min, Number(value))));
}

function normalizeDay() {
  day = clampValue(day, 1, 31);
}

function normalizeMonth() {
  month = clampValue(month, 1, 12);
}

function normalizeYear() {
  year = clampValue(year, currentYear - 100, currentYear);
}
</script>

<div class="grid w-full grid-cols-[1fr_1.35fr_1.6fr] gap-2" dir="ltr">
  <Input
    bind:this={dayInput}
    id={`${idPrefix}-day`}
    type="number"
    min="1"
    max="31"
    inputmode="numeric"
    placeholder={m.common_form_day()}
    bind:value={day}
    oninput={(event: Event) => focusNext(event.currentTarget as HTMLInputElement, 2)}
    onblur={normalizeDay}
    disabled={disabled}
    aria-label={m.common_form_day()}
    class="h-9 w-full bg-background text-center text-xs"
  />
  <Input
    bind:this={monthInput}
    id={`${idPrefix}-month`}
    type="number"
    min="1"
    max="12"
    inputmode="numeric"
    placeholder={m.common_form_month()}
    bind:value={month}
    oninput={(event: Event) => focusNext(event.currentTarget as HTMLInputElement, 2)}
    onblur={normalizeMonth}
    disabled={disabled}
    aria-label={m.common_form_month()}
    class="h-9 w-full bg-background text-center text-xs"
  />
  <Input
    bind:this={yearInput}
    id={`${idPrefix}-year`}
    type="number"
    min={currentYear - 100}
    max={currentYear}
    inputmode="numeric"
    placeholder={m.common_form_year()}
    bind:value={year}
    onblur={normalizeYear}
    disabled={disabled}
    aria-label={m.common_form_year()}
    class="h-9 w-full bg-background text-center text-xs"
  />
</div>

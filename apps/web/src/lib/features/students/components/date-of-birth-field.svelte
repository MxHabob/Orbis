<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { normalizeDigits } from "$lib/shared/utils/search";

interface Props {
  day?: string;
  month?: string;
  year?: string;
  disabled?: boolean;
  required?: boolean;
  idPrefix?: string;
}

let {
  day = $bindable(""),
  month = $bindable(""),
  year = $bindable(""),
  disabled = false,
  required = false,
  idPrefix = "birth",
}: Props = $props();

const currentYear = new Date().getFullYear();
let dayInput = $state<HTMLInputElement | null>(null);
let monthInput = $state<HTMLInputElement | null>(null);
let yearInput = $state<HTMLInputElement | null>(null);

function focusNext(input: HTMLInputElement, maxLength: number) {
  if (input.value.length >= maxLength) {
    if (input === dayInput) monthInput?.focus();
    if (input === monthInput) yearInput?.focus();
  }
}

function clampValue(value: string, min: number, max: number): string {
  const digits = normalizeDigits(value);
  if (!/^[0-9]+$/.test(digits)) return "";
  return String(Math.min(max, Math.max(min, Number(digits))));
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

<div class="grid w-full grid-cols-[1fr_1.35fr_1.6fr] gap-2">
  <Input
    bind:ref={dayInput}
    id={`${idPrefix}-day`}
    type="text"
    maxlength={2}
    inputmode="numeric"
    placeholder={m.common_form_day()}
    bind:value={day}
    oninput={(event: Event) => focusNext(event.currentTarget as HTMLInputElement, 2)}
    onblur={normalizeDay}
    disabled={disabled}
    aria-label={m.common_form_day()}
    aria-required={required}
    class="h-9 w-full bg-background text-center text-xs"
  />
  <Input
    bind:ref={monthInput}
    id={`${idPrefix}-month`}
    type="text"
    maxlength={2}
    inputmode="numeric"
    placeholder={m.common_form_month()}
    bind:value={month}
    oninput={(event: Event) => focusNext(event.currentTarget as HTMLInputElement, 2)}
    onblur={normalizeMonth}
    disabled={disabled}
    aria-label={m.common_form_month()}
    aria-required={required}
    class="h-9 w-full bg-background text-center text-xs"
  />
  <Input
    bind:ref={yearInput}
    id={`${idPrefix}-year`}
    type="text"
    maxlength={4}
    inputmode="numeric"
    placeholder={m.common_form_year()}
    bind:value={year}
    onblur={normalizeYear}
    disabled={disabled}
    aria-label={m.common_form_year()}
    aria-required={required}
    class="h-9 w-full bg-background text-center text-xs"
  />
</div>

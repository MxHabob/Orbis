<!--
@component InputColor
A styled input box with an integrated color indicator, native color picker trigger,
and automatic color hashing from the input text.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";

let {
  id,
  placeholder,
  value = $bindable(""),
  color = $bindable(""),
  autoFocus = false,
  required = false,
  disabled = false,
  onchange,
  onvalueinput,
  onblur,
  ariaInvalid,
  ariaDescribedBy,
}: {
  id?: string;
  placeholder: string;
  value?: string;
  color?: string;
  autoFocus?: boolean;
  required?: boolean;
  disabled?: boolean;
  onchange?: (values: { name: string; color: string }) => void;
  onvalueinput?: (value: string) => void;
  onblur?: () => void;
  ariaInvalid?: "true";
  ariaDescribedBy?: string;
} = $props();

const colors = [
  "#FF6900",
  "#FCB900",
  "#00D084",
  "#8ED1FC",
  "#0693E3",
  "#ABB8C3",
  "#EB144C",
  "#F78DA7",
  "#9900EF",
  "#0079BF",
  "#B6BBBF",
  "#FF5A5F",
  "#F7C59F",
  "#8492A6",
  "#4D5055",
  "#AF5A50",
  "#F9D6E7",
  "#B5EAEA",
  "#B388EB",
  "#B04632",
  "#FF78CB",
  "#4E5A65",
  "#01FF70",
  "#85144b",
  "#F012BE",
  "#7FDBFF",
  "#3D9970",
  "#AAAAAA",
  "#111111",
  "#0074D9",
  "#39CCCC",
  "#001f3f",
  "#FF9F1C",
  "#5E6A71",
  "#75D701",
  "#B6C8A9",
  "#00A9FE",
  "#EAE8E1",
  "#CD346C",
  "#FF6FA4",
  "#D667FB",
  "#0080FF",
  "#656D78",
  "#F8842C",
  "#FF8CFF",
  "#647F6A",
  "#5E574E",
  "#EF5466",
  "#B0E0E6",
  "#EB5E7C",
  "#8A2BE2",
  "#6B7C85",
  "#8C92AC",
  "#6C587A",
  "#52A1FF",
  "#32CD32",
  "#E04F9F",
  "#915C83",
  "#4C6B88",
  "#587376",
  "#C46210",
  "#65B0D0",
  "#2F4F4F",
  "#528B8B",
  "#8B4513",
  "#4682B4",
  "#CD853F",
  "#FFA07A",
  "#CD5C5C",
  "#483D8B",
  "#696969",
];

function customHash(val: string) {
  let hash = 0;
  for (let i = 0; i < val.length; i++) {
    hash = (hash << 5) + val.charCodeAt(i);
    hash = hash & hash;
  }
  return Math.abs(hash);
}

function getColorFromName(val: string) {
  if (!val) return colors[0];
  const hashValue = customHash(val);
  const index = hashValue % colors.length;
  return colors[index];
}

let isCustomColor = $state(false);

function handleInput(e: Event) {
  const target = e.target as HTMLInputElement;
  value = target.value;
  if (!isCustomColor) {
    color = getColorFromName(value);
  }
  onvalueinput?.(value);
  onchange?.({ name: value, color });
}

let colorInputRef: HTMLInputElement | undefined = $state();

function triggerColorPicker() {
  colorInputRef?.click();
}

function handleColorChange(e: Event) {
  const target = e.target as HTMLInputElement;
  color = target.value;
  isCustomColor = true;
  onchange?.({ name: value, color });
}

$effect(() => {
  if (!color) {
    color = colors[Math.floor(Math.random() * colors.length)];
  }
});
</script>

<div class="relative w-full flex items-center">
  <!-- Color trigger button -->
  <button
    type="button"
    onclick={triggerColorPicker}
    class="absolute left-2.5 w-4 h-4 rounded-full border border-border/80 shadow-sm transition-transform active:scale-95 cursor-pointer shrink-0 z-10"
    style="background-color: {color};"
    title={m.shared_ui_choose_color()}
  ></button>

  <!-- Hidden color input -->
  <input
    bind:this={colorInputRef}
    type="color"
    onchange={handleColorChange}
    class="sr-only invisible w-0 h-0"
  />

  <Input
    {id}
    type="text"
    dir="rtl"
    placeholder={placeholder}
    {value}
    oninput={handleInput}
    onblur={onblur}
    autofocus={autoFocus}
    {required}
    {disabled}
    aria-invalid={ariaInvalid}
    aria-describedby={ariaDescribedBy}
    class="pl-9 pe-9 w-full"
    autocomplete="off"
  />
</div>

<script lang="ts">
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";

type Option = {
  value: string;
  label: string;
};

let {
  value,
  options,
  onchange,
  class: className = "",
}: {
  value: string;
  options: Option[];
  onchange: (value: string) => void;
  class?: string;
} = $props();

const selectedLabel = $derived(
  options.find((option) => option.value === value)?.label ?? value,
);
</script>

<Select
  type="single"
  {value}
  onValueChange={(nextValue) => {
    if (nextValue !== undefined) onchange(nextValue);
  }}
>
  <SelectTrigger class={`h-8 w-full text-xs ${className}`}>
    {selectedLabel}
  </SelectTrigger>
  <SelectContent>
    {#each options as option (option.value)}
      <SelectItem value={option.value}>{option.label}</SelectItem>
    {/each}
  </SelectContent>
</Select>

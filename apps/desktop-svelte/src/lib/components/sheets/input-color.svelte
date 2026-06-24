<!--
@component InputColor
A styled input box with an integrated color indicator and native color picker trigger.
-->
<script lang="ts">
import InputColorShared from "../shared/input-color.svelte";

let {
  placeholder,
  value = $bindable(""),
  color = $bindable(""),
  autoFocus = false,
  required = false,
}: {
  placeholder: string;
  value?: string;
  color?: string;
  autoFocus?: boolean;
  required?: boolean;
} = $props();
</script>

<InputColorShared
  {placeholder}
  bind:value
  bind:color
  {autoFocus}
  {required}
/>


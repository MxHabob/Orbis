<script lang="ts">
import type { SVGAttributes } from "svelte/elements";
import { cn } from "../utils";
interface Props extends SVGAttributes<SVGSVGElement> {
  class?: string;
  className?: string;
  size?: number | string;
  strokeWidth?: number | string;
}
let {
  class: klass = "",
  className = "",
  size,
  strokeWidth,
  ...restProps
}: Props = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<rect x="3" y="2.5" width="18" height="11" rx="1.2" />
	<path d="M5 4.5h14v7H5zM8 20.5h8m-4-7v7M6 16h4v2H6zm8 0h4v2h-4z" />
	<path d="m8 9 2-2 2 2 2-3 2 3m-9 7v-1.5m9 1.5v-1.5M4 22l2-4m14 4-2-4" />
</svg>

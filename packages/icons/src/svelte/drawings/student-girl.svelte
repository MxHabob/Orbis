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
	<path d="M5 12.5c-.8-5.9 2-10 7-10s7.8 4.1 7 10l-1.2 2.7-1.2-1.8H7.4l-1.2 1.8L5 12.5Z" />
	<path d="M7.1 10.5a4.9 4.9 0 0 1 9.8 0v4.1a4.9 4.9 0 0 1-9.8 0v-4.1Zm2.2-3.1c2.1.2 4-.4 5.4-1.8m-6.3 7h.1m6.8 0h.1m-4.3 3.1c1 .8 2 .8 3 0" />
	<circle cx="9.5" cy="12.6" r=".35" fill="currentColor" />
	<circle cx="14.5" cy="12.6" r=".35" fill="currentColor" />
	<path d="M3 22c.6-3.6 3.5-5.5 9-5.5s8.4 1.9 9 5.5H3Z" />
	<path d="m9 17 3 3 3-3m-3 3v2M5.1 7.3l-2-1.5m15.8 1.5 2-1.5" />
</svg>

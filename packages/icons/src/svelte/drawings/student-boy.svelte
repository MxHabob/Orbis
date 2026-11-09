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
	<path d="M5.5 9.5c-.2-4.2 2.2-7 6.5-7s6.8 2.8 6.5 7l-1 6.3a5.6 5.6 0 0 1-11 0l-1-6.3Z" />
	<path d="M5.8 8.5c.3-3.1 2.2-5.2 5.1-5.8-.4 1.5-1.4 2.4-2.7 2.9 2.8-.2 4.5-1.3 5.6-3 2.8 1.1 4.2 3.2 4.4 6m-9.7 3.7h.1m6.8 0h.1m-4.3 3.2c1 .8 2 .8 3 0" />
	<circle cx="8.5" cy="12.2" r=".35" fill="currentColor" />
	<circle cx="15.5" cy="12.2" r=".35" fill="currentColor" />
	<path d="M3 22c.6-3.6 3.5-5.5 9-5.5s8.4 1.9 9 5.5H3Z" />
	<path d="m9 17 3 3 3-3m-3 3v2" />
</svg>

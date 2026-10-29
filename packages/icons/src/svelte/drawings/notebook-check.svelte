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
	<path d="M6 3.5h10.5l4 4v12A1.5 1.5 0 0 1 19 21H6a1.5 1.5 0 0 1-1.5-1.5V5A1.5 1.5 0 0 1 6 3.5Z" />
	<path d="M16.5 3.8v4h3.8M8 8.5h4m-4 3h3.5m-3.5 3h2.5" />
	<circle cx="16.5" cy="15.4" r="3.3" />
	<path d="m15.1 15.4 1 1 1.9-2.1M3.3 5.5h2m-2 3h2m-2 3h2m-2 3h2m-2 3h2" />
</svg>

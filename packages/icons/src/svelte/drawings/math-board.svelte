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
	<rect x="2.5" y="3.5" width="19" height="14.5" rx="1.8" />
	<path d="M4.5 5.5h15v10.5h-15zM8 20.5h8m-4-2.5v2.5" />
	<path d="m7 11 1.5 1.5 1.2-4h2.1m1.8 3.9 2.7-2.7m0 2.7-2.7-2.7" />
	<path d="M5 19h2l.6-1h8.8l.6 1h2" />
</svg>

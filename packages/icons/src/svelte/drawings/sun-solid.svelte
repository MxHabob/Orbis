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
	<circle cx="12" cy="12" r="4.2" fill="currentColor" />
	<path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" />
</svg>

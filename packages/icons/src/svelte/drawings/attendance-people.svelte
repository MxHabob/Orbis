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
	<circle cx="12" cy="5" r="2.5" />
	<circle cx="5" cy="9" r="2" />
	<circle cx="19" cy="9" r="2" />
	<circle cx="7.5" cy="5.5" r="1.5" />
	<circle cx="16.5" cy="5.5" r="1.5" />
	<path d="M6.5 20v-2.5a5.5 5.5 0 0 1 11 0V20H6.5Zm-1-8.8a3.5 3.5 0 0 0-3 3.5V18h3m13 0h3v-3.3a3.5 3.5 0 0 0-3-3.5m-7.4 1.3a4.5 4.5 0 0 0-3.2-1.3m7.2 0a4.5 4.5 0 0 1 3.2 1.3" />
</svg>

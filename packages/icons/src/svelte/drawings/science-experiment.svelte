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
	<path d="M7.8 3.1c2.3 2.4 2.3 5.4 0 8.1-2.3 2.7-5.3 2.9-7 .5m2.1-7.9c2.4-.2 4.7 1.6 6 4.7 1.3 3.1.8 6.1-1.4 7.4M1 8.7c3.1-1.2 6-.4 8.1 2.2 2.1 2.6 2.1 5.6-.1 7.6" />
	<circle cx="4.7" cy="11.3" r="1.2" />
	<path d="M15.6 3.5h5m-4 .1v5.1l-4.1 9.1a1.7 1.7 0 0 0 1.5 2.4h5.6a1.7 1.7 0 0 0 1.5-2.4L17 8.7" />
	<path d="M14.2 15.1c1.8.8 3.6.8 5.5 0m-4.6 2.3.1.1m2.9-1.4.1.1" />
</svg>

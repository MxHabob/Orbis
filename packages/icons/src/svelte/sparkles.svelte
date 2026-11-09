<script lang="ts">
import type { SVGAttributes } from "svelte/elements";
import { cn } from "./utils";

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

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width={strokeWidth ?? '1.75'}
	width={size}
	height={size}
	class={cn(className || (size === undefined ? 'size-4' : ''), klass)}
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
	{...restProps}>
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M9 4 10.8 9.2 16 11l-5.2 1.8L9 18l-1.8-5.2L2 11l5.2-1.8L9 4Zm9-1.5 1.1 3.5L22.5 7l-3.4 1-1.1 3.5-1.1-3.5L13.5 7l3.4-1L18 2.5Zm-1 13 0.8 3.2 3.2 0.8-3.2 0.8L17 24l-0.8-3.2L13 20l3.2-0.8L17 15.5Z"
	/>
</svg>

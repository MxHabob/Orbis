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
		d="M20 11a8 8 0 0 0-13.66-5.66L4 7.68M4 4.5v3.18h3.18M4 13a8 8 0 0 0 13.66 5.66L20 16.32M20 19.5v-3.18h-3.18"
	/>
</svg>

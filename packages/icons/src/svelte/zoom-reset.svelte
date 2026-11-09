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
	<rect x="2" y="4" width="20" height="16" rx="3" fill="none" />
	<!-- The "1" -->
	<path stroke-linecap="round" stroke-linejoin="round" d="M7.5 9v6M6 10.5l1.5-1.5" />
	<!-- The ":" -->
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M12 10.5v.01M12 13.5v.01"
		stroke-width="2"
	/>
	<!-- The other "1" -->
	<path stroke-linecap="round" stroke-linejoin="round" d="M16.5 9v6M15 10.5l1.5-1.5" />
</svg>

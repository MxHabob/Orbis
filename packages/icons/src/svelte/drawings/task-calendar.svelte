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
	<path d="M7.5 6V3.75m4.5 1.85V3.35m4.5 1.85V3" />
	<path
		d="m5.7 5.65 13.55-1.2a1.5 1.5 0 0 1 1.62 1.36l1.02 11.48a1.5 1.5 0 0 1-1.36 1.62L7 20.1a1.5 1.5 0 0 1-1.62-1.36L4.36 7.27A1.5 1.5 0 0 1 5.7 5.65Z"
	/>
	<path d="m5.05 9.1 16.26-1.43" />
	<rect x="7.3" y="11" width="2.5" height="2.5" rx=".35" />
	<rect x="11.2" y="10.65" width="2.5" height="2.5" rx=".35" />
	<rect x="7.65" y="15.1" width="2.5" height="2.5" rx=".35" />
	<path d="m13.4 15.05 1.65 1.55 3.15-4.15" />
</svg>

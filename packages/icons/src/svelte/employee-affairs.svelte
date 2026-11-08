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
	<rect x="3.25" y="3.5" width="17.5" height="17" rx="2.75" />
	<circle cx="8.5" cy="9" r="2.25" />
	<path d="M5.5 16a3 3 0 0 1 6 0m2-5.75h4.25M13.5 14h4.25m-4.25 3.75h2.5" />
</svg>

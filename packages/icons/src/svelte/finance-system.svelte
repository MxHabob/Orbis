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
	<path d="M4.25 4.25h10.5v15.5H4.25A1.25 1.25 0 0 1 3 18.5V5.5a1.25 1.25 0 0 1 1.25-1.25Z" />
	<path d="M6.5 8h5.75m-5.75 3.25h4.5m-4.5 3.25h3" />
	<circle cx="17.25" cy="16.75" r="4.25" />
	<path d="M16 15.75h2.5m-2.5 2h2.5" />
</svg>

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
	<path d="M2.75 7.5 12 3l9.25 4.5L12 12 2.75 7.5Z" />
	<path d="M6.5 9.5v3.75c3.3 2.2 7.7 2.2 11 0V9.5m3.75-2v5" />
	<path d="M4.5 16c2.65-.75 5.15-.35 7.5 1.1 2.35-1.45 4.85-1.85 7.5-1.1v4.25c-2.65-.7-5.15-.35-7.5 1-2.35-1.35-4.85-1.7-7.5-1V16Zm7.5 1.1v4.15" />
</svg>

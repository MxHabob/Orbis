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
	<path d="M7 3.75h10A3.25 3.25 0 0 1 20.25 7v8A3.25 3.25 0 0 1 17 18.25h-5L7.5 21v-2.75A3.25 3.25 0 0 1 3.75 15V7A3.25 3.25 0 0 1 7 3.75Z" />
	<path d="m12 7.25.95 2.8 2.8.95-2.8.95-.95 2.8-.95-2.8-2.8-.95 2.8-.95.95-2.8Z" />
	<circle cx="6.75" cy="7.25" r=".45" fill="currentColor" stroke="none" />
	<circle cx="17.25" cy="14.75" r=".45" fill="currentColor" stroke="none" />
</svg>

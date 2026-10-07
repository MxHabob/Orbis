<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from './utils';

	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string;
		className?: string;
		size?: number | string;
		strokeWidth?: number | string;
	}

	let {
		class: klass = '',
		className = '',
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
	<path d="M6 3.25h8l4.75 4.5v12A1.25 1.25 0 0 1 17.5 21h-11A1.25 1.25 0 0 1 5.25 19.75V4.5A1.25 1.25 0 0 1 6.5 3.25Z" />
	<path d="M14 3.75v4.5h4.25M8.25 11.5h3v3h-3zm5 0h2.5v3h-2.5zm-5 5h3v2h-3zm5 0h2.5v2h-2.5z" />
</svg>

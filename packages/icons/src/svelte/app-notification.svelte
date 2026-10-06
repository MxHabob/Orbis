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
	class={cn(className || 'size-4', klass)}
	aria-hidden="true"
	{...restProps}
>
	<path
		d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
	<path
		d="M10.3 21a1.94 1.94 0 0 0 3.4 0"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

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
	stroke-width={strokeWidth ?? '1.5'}
	width={size}
	height={size}
	class={cn(className || 'size-4', klass)}
	aria-hidden="true"
	{...restProps}
>
	<path d="M10.25 4.75L6.75 8.25L10.25 11.75" stroke-linecap="round" stroke-linejoin="round"
	></path><path
		d="M6.75 8.25L12.75 8.25C14.9591 8.25 16.75 10.0409 16.75 12.25V19.25"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

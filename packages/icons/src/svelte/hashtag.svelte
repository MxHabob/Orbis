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
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
	{...restProps}>
	<path d="M10 3L6 21" stroke-linecap="round"></path><path d="M20.5 16H2.5" stroke-linecap="round"
	></path><path d="M22 7H4" stroke-linecap="round"></path><path
		d="M18 3L14 21"
		stroke-linecap="round"
	></path>
</svg>

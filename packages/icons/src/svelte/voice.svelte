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
	fill="currentColor"
	stroke="currentColor"
	stroke-width={strokeWidth ?? '1.75'}
	width={size}
	height={size}
	class={cn(className || 'size-4', klass)}
	aria-hidden="true"
	{...restProps}

	stroke-linecap="round"
	stroke-linejoin="round">
	<path d="M12 4L12 20" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M8 9L8 15"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M20 10L20 14" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M4 10L4 14"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M16 7L16 17" stroke-linecap="round" stroke-linejoin="round"></path>
</svg>

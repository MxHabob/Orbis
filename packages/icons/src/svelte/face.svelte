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
	<path
		d="M12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2Z"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
	<path d="M16 8L16 10" stroke-linecap="round" stroke-linejoin="round"></path>

	<path d="M8 8L8 10" stroke-linecap="round" stroke-linejoin="round"></path>

	<path d="M9 16C9 16 10 17 12 17C14 17 15 16 15 16" stroke-linecap="round" stroke-linejoin="round"
	></path>

	<path d="M12 8L12 13L11 13" stroke-linecap="round" stroke-linejoin="round"></path>
</svg>

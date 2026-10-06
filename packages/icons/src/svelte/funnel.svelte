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
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M12 3c2.755 0 5.455.232 8.083.678.539.092.917.56.917 1.108v1.396a2.25 2.25 0 0 1-.659 1.591L15 13.064V19.5a2.25 2.25 0 0 1-.879 1.795l-3 2.25A2.25 2.25 0 0 1 9.75 22.5v-9.436L4.659 7.773A2.25 2.25 0 0 1 4 6.182V4.786c0-.548.378-1.016.917-1.108A48.507 48.507 0 0 1 12 3Z"
	/>
</svg>

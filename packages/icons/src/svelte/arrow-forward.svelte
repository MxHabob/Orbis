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
	<path stroke-linecap="round" stroke-linejoin="round" d="M13.25 19.25L16.75 15.75L13.25 12.25" />
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M16.75 15.75H10.75C8.54086 15.75 6.75 13.9591 6.75 11.75V7.5"
	/>
</svg>

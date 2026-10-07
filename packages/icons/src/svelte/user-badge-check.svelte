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

	stroke-linecap="round"
	stroke-linejoin="round">
	<circle cx="7.5" cy="7.5" r="3.25" />
	<path d="M1.75 20v-1.25a5.75 5.75 0 0 1 11.5 0V20H1.75Z" stroke-linecap="round" stroke-linejoin="round" />
	<circle cx="18" cy="17" r="4.25" />
	<path d="m16.25 17 1.2 1.2 2.35-2.5" stroke-linecap="round" stroke-linejoin="round" />
</svg>

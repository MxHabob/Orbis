<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';

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

	const computedClass = $derived([className || 'size-4', klass].filter(Boolean).join(' '));
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width={strokeWidth ?? '1.5'}
	width={size}
	height={size}
	class={computedClass}
	aria-hidden="true"
	{...restProps}
>
	<path d="M11 6H21" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M11 12H21"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M11 18H21" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M5 19V5M5 19L3 16.5M5 19L7 16.5M5 5L3 7M5 5L7 7"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

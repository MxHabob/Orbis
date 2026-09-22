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
	stroke-width={strokeWidth ?? 'strokeWidth'}
	width={size}
	height={size}
	class={computedClass}
	aria-hidden="true"
	{...restProps}
>
	<path d="M9.5 14.5L3 21" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M5.00007 9.48528L14.1925 18.6777L15.8895 16.9806L15.4974 13.1944L21.0065 8.5211L15.1568 2.67141L10.4834 8.18034L6.69713 7.78823L5.00007 9.48528Z"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

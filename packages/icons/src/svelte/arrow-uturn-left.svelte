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
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3"
	/>
</svg>

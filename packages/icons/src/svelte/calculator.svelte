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
		d="M15.75 15.75h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-3-2.25h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-3-2.25h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-1.5-6.75h9m-9-3h9M4.5 19.5V4.5A2.25 2.25 0 0 1 6.75 2.25h10.5A2.25 2.25 0 0 1 19.5 4.5v15A2.25 2.25 0 0 1 17.25 21.75H6.75A2.25 2.25 0 0 1 4.5 19.5Z"
	/>
</svg>

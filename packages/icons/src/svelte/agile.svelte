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
		d="M17.5 19H22M22 19L19.5 16.5M22 19L19.5 21.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M12 2L9.5 4.5L12 7" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M10.5 4.5C14.6421 4.5 18 7.85786 18 12C18 16.1421 14.6421 19.5 10.5 19.5H2"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path
		d="M6.75583 5.5C4.51086 6.79595 3 9.22154 3 12C3 13.6884 3.55792 15.2465 4.49945 16.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

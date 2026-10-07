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
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
	{...restProps}>
	<!-- Left lens -->
	<path
		d="M2 14
		   C2 16.2091 3.79086 18 6 18
		   C8.20914 18 10 16.2091 10 14
		   C10 11.7909 8.20914 10 6 10
		   C3.79086 10 2 11.7909 2 14Z"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>

	<!-- Right lens -->
	<path
		d="M14 14
		   C14 16.2091 15.7909 18 18 18
		   C20.2091 18 22 16.2091 22 14
		   C22 11.7909 20.2091 10 18 10
		   C15.7909 10 14 11.7909 14 14Z"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>

	<!-- Left temple (side-mounted) -->
	<path d="M2 14L0.8 14.4" stroke-linecap="round" />

	<!-- Right temple (side-mounted) -->
	<path d="M22 14L23.2 14.4" stroke-linecap="round" />

	<!-- Bridge -->
	<path d="M14 14H10" stroke-linecap="round" stroke-linejoin="round" />
</svg>

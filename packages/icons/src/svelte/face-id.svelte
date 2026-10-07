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
	<path d="M7 3H5C3.89543 3 3 3.89543 3 5V7" stroke-linecap="round" stroke-linejoin="round"
	></path><path
		d="M17 3H19C20.1046 3 21 3.89543 21 5V7"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M16 8L16 10" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M8 8L8 10"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path
		d="M9 16C9 16 10 17 12 17C14 17 15 16 15 16"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path d="M12 8L12 13L11 13" stroke-linecap="round" stroke-linejoin="round"></path><path
		d="M7 21H5C3.89543 21 3 20.1046 3 19V17"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path><path
		d="M17 21H19C20.1046 21 21 20.1046 21 19V17"
		stroke-linecap="round"
		stroke-linejoin="round"
	></path>
</svg>

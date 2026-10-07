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
	fill="currentColor"
	stroke="currentColor"
	stroke-width={strokeWidth ?? '1.75'}
	width={size}
	height={size}
	class={cn(className || 'size-4', klass)}
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
	{...restProps}>
	<path
		fill-rule="evenodd"
		stroke="none"
		d="M6.25 2.5h7.3l5.95 5.95v11.8A1.75 1.75 0 0 1 17.75 22h-11A1.75 1.75 0 0 1 5 20.25V3.75A1.25 1.25 0 0 1 6.25 2.5ZM14 4.05v3.2c0 .55.45 1 1 1h3.2L14 4.05ZM12 10.2l-4.2 4.2h2.7v3.8h3v-3.8h2.7L12 10.2Z"
	/>
</svg>

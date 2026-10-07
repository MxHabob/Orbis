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
	<rect x="3.5" y="3.5" width="7" height="8.5" rx="1.25" />
	<rect x="13.5" y="3.5" width="7" height="5.5" rx="1.25" />
	<rect x="3.5" y="15" width="7" height="5.5" rx="1.25" />
	<rect x="13.5" y="12" width="7" height="8.5" rx="1.25" />
</svg>

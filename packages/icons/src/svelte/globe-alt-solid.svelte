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
	class={cn(className || (size === undefined ? 'size-4' : ''), klass)}
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
	{...restProps}>
	<path
		fill-rule="evenodd"
		stroke="none"
		d="M12 2.75a9.25 9.25 0 1 0 0 18.5a9.25 9.25 0 0 0 0-18.5ZM12 4.5c.98 1.25 1.48 3.45 1.45 6.25h-2.9C10.52 7.95 11.02 5.75 12 4.5ZM10.55 13.25h2.9c.03 2.8-.47 5-1.45 6.25c-.98-1.25-1.48-3.45-1.45-6.25ZM3.4 11c1.9.7 4.1 1 6.4 1v1.5c-2.3 0-4.5-.3-6.4-1V11Zm11 1c2.3 0 4.5-.3 6.2-1v1.5c-1.7.7-3.9 1-6.2 1V12Z"
	/>
</svg>

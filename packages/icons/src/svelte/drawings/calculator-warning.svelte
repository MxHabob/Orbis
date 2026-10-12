<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from '../utils';
	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string; className?: string; size?: number | string; strokeWidth?: number | string;
	}
	let { class: klass = '', className = '', size, strokeWidth, ...restProps }: Props = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<path d="m2.2 18.6 5.5-12a1.2 1.2 0 0 1 2.2 0l5.4 12a1.2 1.2 0 0 1-1.1 1.7H3.3a1.2 1.2 0 0 1-1.1-1.7Z" />
	<path d="m7.1 11 2.7 4.8m0-4.8-2.7 4.8m1.35-8v1" />
	<rect x="14.5" y="4" width="7" height="16" rx="1.4" />
	<rect x="16.1" y="5.8" width="3.8" height="2.6" rx=".3" />
	<path d="M16.4 11.2h.1m2.7 0h.1m-2.9 2.6h.1m2.7 0h.1m-2.9 2.6h.1m2.7 0h.1" />
</svg>

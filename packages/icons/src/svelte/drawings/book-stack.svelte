<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from '../utils';
	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string; className?: string; size?: number | string; strokeWidth?: number | string;
	}
	let { class: klass = '', className = '', size, strokeWidth, ...restProps }: Props = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<path d="m3 8.2 12.8-3 5.2 2.4-12.8 3L3 8.2Z" />
	<path d="M3 8.2v2.7l5.2 2.3 12.8-3V7.6M4.2 13.2l11.6-2.7 4.9 2.3-11.6 2.7-4.9-2.3Z" />
	<path d="M4.2 13.2v2.5l4.9 2.4 11.6-2.7v-2.6m-13 5.5 1.4 1.1 11.4-2.6M6.6 8.9v2.7m.9 3.1v2.5" />
</svg>

<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from '../utils';
	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string; className?: string; size?: number | string; strokeWidth?: number | string;
	}
	let { class: klass = '', className = '', size, strokeWidth, ...restProps }: Props = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<path d="M8 6.5V5a4 4 0 0 1 8 0v1.5m-8 0A3.5 3.5 0 0 0 4.5 10v9A2.5 2.5 0 0 0 7 21.5h10a2.5 2.5 0 0 0 2.5-2.5v-9A3.5 3.5 0 0 0 16 6.5H8Z" />
	<path d="M4.7 11h14.6m-13.8 0v7.5a1.5 1.5 0 0 0 1.5 1.5h9.9a1.5 1.5 0 0 0 1.5-1.5V11" />
	<rect x="7.2" y="13" width="9.6" height="5" rx="1.3" />
	<path d="M10.5 13v-1m3 1v-1m-1.5 2.4.3.7.8.1-.6.5.1.8-.6-.4-.7.4.2-.8-.6-.5.8-.1.3-.7ZM3 12l1.5 1m16.5-1-1.5 1" />
</svg>

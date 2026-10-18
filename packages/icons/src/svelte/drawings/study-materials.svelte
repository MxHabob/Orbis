<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from '../utils';

	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string;
		className?: string;
		size?: number | string;
		strokeWidth?: number | string;
	}

	let { class: klass = '', className = '', size, strokeWidth, ...restProps }: Props = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<path d="m3 17.2 11.8-2.4 5.2 2.1-11.8 2.5L3 17.2Z" />
	<path d="m3 17.2.2 2.2 5 2.1 11.8-2.5v-2.1" />
	<path d="m4 14.2 10.9-2.3 4.8 2-10.9 2.3L4 14.2Z" />
	<path d="m4 14.2.2 2.2 4.6 1.8 10.9-2.3v-2" />
	<path d="m5.1 11.5 9.8-2.1 4.4 1.8-9.8 2.1-4.4-1.8Z" />
	<rect x="7" y="6" width="1.8" height="5" rx=".25" />
	<path d="M10.3 5.2v5.4m2.2-6.3v6.1m2.2-7.2v7.8" />
	<path d="m17.4 2.8 4.1-.5.7 5.4-4.1.5-.7-5.4Z" />
	<path d="m18.5 4.1 2.1-.3m-2 1.6 1.4-.2m.3 1.6.1.5" />
</svg>

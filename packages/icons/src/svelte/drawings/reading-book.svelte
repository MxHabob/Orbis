<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from '../utils';
	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string; className?: string; size?: number | string; strokeWidth?: number | string;
	}
	let { class: klass = '', className = '', size, strokeWidth, ...restProps }: Props = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={strokeWidth ?? '1.75'} width={size} height={size} class={cn(className || (size === undefined ? 'size-4' : ''), klass)} stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {...restProps}>
	<path d="M3 5.1c3.5-1.2 6.4-.7 9 1.4 2.6-2.1 5.5-2.6 9-1.4v13c-3.5-1.2-6.4-.7-9 1.4-2.6-2.1-5.5-2.6-9-1.4v-13Z" />
	<path d="M12 6.5v13M5.3 8.1c1.8-.4 3.2-.1 4.6.7m-4.6 2c1.8-.4 3.2-.1 4.6.7m-4.6 2c1.8-.4 3.2-.1 4.6.7m4.1-5.4c1.4-.8 2.8-1.1 4.6-.7m-4.6 2.7c1.4-.8 2.8-1.1 4.6-.7m-4.6 2.7c1.4-.8 2.8-1.1 4.6-.7" />
	<path d="M2 6.8v12.4c3.6-1.1 6.8-.6 10 1.6 3.2-2.2 6.4-2.7 10-1.6V6.8" />
</svg>

<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';
	import { cn } from './utils';

	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string;
		className?: string;
		dotClass?: string;
		size?: number | string;
	}

	let {
		class: klass = '',
		className = '',
		dotClass = '',
		size,
		...restProps
	}: Props = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 642 660"
	fill="none"
	width={size}
	height={size}
	class={cn(className || 'size-4', klass)}
	aria-hidden="true"
	{...restProps}
>
	<path
		fill-rule="evenodd"
		clip-rule="evenodd"
		d="M266.088 4.14842C13.2491 48.4308 -88.4636 368.722 89.8783 558.989C266.569 747.489 572.837 659.084 633.009 402.221C653.563 314.469 638.492 266.298 591.748 270.345C565.093 272.658 552.235 289.064 552.235 320.761C552.235 509.135 362.999 628.727 208.025 538.302C-1.18046 416.23 85.0444 90.7106 326.588 90.7106C377.228 90.7106 393.966 28.5226 349.691 4.89411C338.035 -1.32301 299.284 -1.66652 266.088 4.14842Z"
		fill="currentColor"
	/>
	<path
		d="M461.208 126.279C450.979 89.5042 473.161 48.4644 509.122 37.6473C570.32 19.2473 622.788 82.9853 594.489 141.361C565.366 201.438 479.405 191.71 461.208 126.279Z"
		fill={dotClass ? undefined : '#A821D9'}
		class={dotClass}
	/>
</svg>

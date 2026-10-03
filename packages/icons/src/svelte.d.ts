declare module '*.svelte' {
	import type { Component } from 'svelte';
	import type { SVGAttributes } from 'svelte/elements';

	interface Props extends SVGAttributes<SVGSVGElement> {
		class?: string;
		className?: string;
		size?: number | string;
		strokeWidth?: number | string;
	}

	const component: Component<Props>;
	export default component;
}

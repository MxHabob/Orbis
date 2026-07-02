<script lang="ts">
import { Avatar as AvatarPrimitive } from "bits-ui";
import { cn } from "../../../utils.js";

let {
  ref = $bindable(null),
  class: className,
  ...restProps
}: AvatarPrimitive.FallbackProps = $props();
</script>

<AvatarPrimitive.Fallback
	bind:ref
	data-slot="avatar-fallback"
	class={cn(
		"absolute inset-0 bg-brand-pink text-white rounded-[inherit] flex items-center justify-center text-sm font-bold group-data-[size=sm]/avatar:text-xs",
		className
	)}
	{...restProps}
/>



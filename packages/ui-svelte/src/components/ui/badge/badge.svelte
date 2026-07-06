<script lang="ts" module>
import { type VariantProps, tv } from "tailwind-variants";

export const badgeVariants = tv({
  base: "gap-1 has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&>svg]:size-3! focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive group/badge inline-flex w-fit shrink-0 items-center justify-center overflow-hidden whitespace-nowrap focus-visible:ring-[3px] [&>svg]:pointer-events-none",
  variants: {
    variant: {
      default: "badge-brand-primary",
      secondary: "badge-brand-default",
      destructive:
        "badge-brand-destructive focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40",
      outline: "border-border text-foreground bg-background rounded-md px-2 py-0.5 text-[10px] font-semibold border",
      ghost: "bg-transparent text-muted-foreground border-transparent rounded-md px-2 py-0.5 text-[10px] font-semibold",
      lime: "badge-brand-lime",
      brand: "badge-brand-brand",
      link: "text-primary underline underline-offset-4 text-[10px] font-semibold",
      success: "badge-brand-success",
      failure: "badge-brand-failure",
      warning: "badge-brand-warning",
      info: "badge-brand-info",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

export type BadgeVariant = VariantProps<typeof badgeVariants>["variant"];
</script>

<script lang="ts">
	import type { HTMLAnchorAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "../../../utils.js";

	let {
		ref = $bindable(null),
		href,
		class: className,
		variant = "default",
		children,
		...restProps
	}: WithElementRef<HTMLAnchorAttributes> & {
		variant?: BadgeVariant;
	} = $props();
</script>

<svelte:element
	this={href ? "a" : "span"}
	bind:this={ref}
	data-slot="badge"
	{href}
	class={cn(badgeVariants({ variant }), className)}
	{...restProps}
>
	{@render children?.()}
</svelte:element>

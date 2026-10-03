import type { WithElementRef } from "../../../utils.js";
import type {
  HTMLAnchorAttributes,
  HTMLButtonAttributes,
} from "svelte/elements";
import { type VariantProps, tv } from "tailwind-variants";

export const buttonVariants = tv({
  base: "focus-visible:border-ring aria-invalid:border-destructive rounded-md border border-transparent bg-clip-padding text-sm font-medium active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-']):not([width])]:size-4 group/button inline-flex shrink-0 items-center justify-center whitespace-nowrap outline-none select-none transition-[background-color,border-color,box-shadow,color,transform] duration-180 ease-out disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
  variants: {
    variant: {
      default: "btn-tactile-primary",
      outline:
        "btn-tactile border-border bg-background hover:bg-muted hover:text-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 aria-expanded:bg-muted aria-expanded:text-foreground shadow-xs",
      secondary:
        "btn-tactile bg-secondary text-secondary-foreground hover:bg-secondary/80 aria-expanded:bg-secondary aria-expanded:text-secondary-foreground",
      ghost: "btn-tactile-ghost",
      destructive: "btn-tactile-destructive",
      link: "text-primary underline-offset-4 hover:underline",
      lime: "btn-tactile-lime",
    },
    size: {
      default:
        "h-8 gap-1.5 px-3 in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
      xs: "h-6 gap-1 rounded-[min(var(--radius-md),8px)] px-2 text-xs in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-']):not([width])]:size-3",
      sm: "h-7 gap-1 rounded-[min(var(--radius-md),10px)] px-2.5 in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5",
      lg: "h-9 gap-1.5 px-3 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
      icon: "w-8 h-8 p-0 flex items-center justify-center shrink-0",
      "icon-xs":
        "size-6 rounded-[min(var(--radius-md),8px)] in-data-[slot=button-group]:rounded-md [&_svg:not([class*='size-']):not([width])]:size-3",
      "icon-sm":
        "w-7 h-7 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-md p-0 flex items-center justify-center shrink-0",
      "icon-lg": "w-9 h-9 p-0 flex items-center justify-center shrink-0",
    },
  },
  defaultVariants: {
    variant: "default",
    size: "default",
  },
});

export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
  WithElementRef<HTMLAnchorAttributes> & {
    variant?: ButtonVariant;
    size?: ButtonSize;
  };

import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { Snippet } from "svelte";
import type { HTMLAttributes } from "svelte/elements";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// ── Type helpers used by shadcn-svelte components ────────────────────

/** Remove the `children` snippet prop from a component's props. */
export type WithoutChildren<T> = Omit<T, "children">;

/** Remove the `child` snippet prop from a component's props. */
export type WithoutChild<T> = Omit<T, "child">;

/** Remove both `children` and `child` snippet props from a component's props. */
export type WithoutChildrenOrChild<T> = Omit<T, "children" | "child">;

/** Add an `el` prop for binding an element ref. */
export type WithElementRef<
	T,
	E extends HTMLElement = HTMLElement,
> = T & { el?: E | null };

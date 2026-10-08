import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import type { Pathname } from "$app/types";
import { localizeHref } from "$lib/paraglide/runtime";

export function navigateOnboardingRoute(path: string) {
  return goto(resolve(localizeHref(path) as Pathname));
}

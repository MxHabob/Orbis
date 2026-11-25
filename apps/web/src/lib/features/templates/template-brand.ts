import { orgContext } from "$lib/stores/org-context.svelte";

export function organizationBrandColor() {
  return /^#[0-9a-f]{6}$/i.test(orgContext.primaryColor) ? orgContext.primaryColor : "#475569";
}

export function resolveTemplateColor(color: string) {
  return color === "org-primary" ? organizationBrandColor() : color;
}

export function templateOrganization() {
  return {
    name: orgContext.name,
    name_ar: orgContext.nameAr,
    logo_url: orgContext.logo,
    primary_color: organizationBrandColor(),
    currency: orgContext.baseCurrency,
  };
}

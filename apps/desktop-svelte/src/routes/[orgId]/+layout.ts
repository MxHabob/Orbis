import type { LayoutLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: LayoutLoad = async ({ params }) => {
  const { authStore } = await import("$lib/stores");
  authStore.restore();
  
  if (!authStore.isAuthenticated) {
    throw redirect(302, "/login");
  }
  
  await authStore.checkSetupStatus();
  if (authStore.needsOnboarding) {
    throw redirect(302, "/onboarding");
  }
  
  return { orgId: params.orgId };
};

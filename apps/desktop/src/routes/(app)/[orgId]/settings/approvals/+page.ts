import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ url }) => {
  const destination = new URL(url);
  destination.pathname = destination.pathname.replace(
    /\/settings\/approvals\/?$/,
    "/approvals",
  );
  redirect(307, destination);
};

import type { LayoutData } from "../$types";

export function load({ params }: { params: { orgId: string } }): LayoutData {
  return { orgId: params.orgId };
}

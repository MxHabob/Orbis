export function load({ params }: { params: { orgId: string } }) {
  return { orgId: params.orgId };
}

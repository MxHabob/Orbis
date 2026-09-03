export function load({ params }: { params: { orgId: string; appId: string } }) {
  return {
    orgId: params.orgId,
    appId: params.appId,
  };
}

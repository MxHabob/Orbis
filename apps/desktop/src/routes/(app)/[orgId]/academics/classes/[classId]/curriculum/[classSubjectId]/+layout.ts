import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ params, parent }) => {
  await parent();
  return {
    orgId: params.orgId,
    classId: params.classId,
    classSubjectId: params.classSubjectId,
  };
};

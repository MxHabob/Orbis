import { parseAsString, useQueryStates } from "nuqs";

export function useStaffParams() {
  const [params, setParams] = useQueryStates({
    deleteStaffId: parseAsString,
    leaveTeamId: parseAsString,
    q: parseAsString,
  });

  return {
    ...params,
    setParams,
  };
}

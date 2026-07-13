import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useAcademicParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    editPeriodId: parseAsString,
    editTermId: parseAsString,
    parentPeriodId: parseAsString,
    deletePeriodId: parseAsString,
    deleteTermId: parseAsString,
    createYear: parseAsBoolean,
    createTerm: parseAsBoolean,
    q: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}

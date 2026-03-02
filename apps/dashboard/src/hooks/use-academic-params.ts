import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useAcademicParams() {
  const [queryStates, setQueryStates] = useQueryStates({
    yearId: parseAsString,
    termId: parseAsString,
    createYear: parseAsBoolean,
    createTerm: parseAsBoolean,
    q: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}

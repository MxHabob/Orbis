import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useSubjectParams() {
  const [params, setParams] = useQueryStates({
    subjectId: parseAsString,
    deleteSubjectId: parseAsString,
    createSubject: parseAsBoolean,

    name: parseAsString,
    q: parseAsString,
  });

  return {
    ...params,
    setParams,
  };
}

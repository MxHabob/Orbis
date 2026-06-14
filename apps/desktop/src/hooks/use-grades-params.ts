import { useQueryStates } from "nuqs";
import { parseAsBoolean, parseAsString } from "nuqs/server";

export function useGradesParams() {
  const [params, setParams] = useQueryStates({
    Id: parseAsString,
    createGrade: parseAsBoolean,
    editGrade: parseAsString,
  });

  return {
    ...params,
    setParams,
  };
}

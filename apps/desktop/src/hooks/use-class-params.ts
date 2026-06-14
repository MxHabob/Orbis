import { useParams } from "react-router-dom";
import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function useClassParams() {
  const pathParams = useParams();

  const [queryStates, setQueryStates] = useQueryStates({
    academicYearId: parseAsString,
    classId: parseAsString,
    deleteClassId: parseAsString,
    createClass: parseAsBoolean,

    name: parseAsString,
    q: parseAsString,
    step: parseAsString,
    type: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}

import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";
import { useParams } from "next/navigation";

export function useClassParams() {
  const pathParams = useParams();
  
  const [queryStates, setQueryStates] = useQueryStates({
    classId: parseAsString,
    createClass: parseAsBoolean,
    name: parseAsString,
    q: parseAsString,
  });

  return {
    ...queryStates,
    setParams: setQueryStates,
  };
}

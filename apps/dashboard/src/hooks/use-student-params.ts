import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";
import { useParams } from "next/navigation";

export function useStudentParams() {
  const pathParams = useParams();
  
  const [queryStates, setQueryStates] = useQueryStates({
    studentId: parseAsString,
    createStudent: parseAsBoolean,
    classId: parseAsString,
    name: parseAsString,
    q: parseAsString,
    details: parseAsBoolean,
  });

  return {
    ...queryStates,
    classId: queryStates.classId || (pathParams?.Id as string),
    setParams: setQueryStates,
  };
}
import { useParams } from "react-router-dom";
import {
  parseAsBoolean,
  parseAsString,
  useQueryStates,
} from "nuqs";

export function useStudentParams() {
  const pathParams = useParams();

  const [queryStates, setQueryStates] = useQueryStates({
    studentId: parseAsString,
    deleteStudentId: parseAsString,
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

import {
  parseAsBoolean,
  parseAsInteger,
  parseAsString,
  useQueryStates,
} from "nuqs";

export function useLessonParams() {
  const [params, setParams] = useQueryStates({
    create: parseAsBoolean,
    update: parseAsBoolean,
    lessonId: parseAsInteger,
    classId: parseAsInteger,
    teacherId: parseAsString,
    subjectId: parseAsInteger,
    timeSlotId: parseAsInteger,
    dayOfWeek: parseAsInteger,
    schedule: parseAsBoolean,
    academicPeriodId: parseAsInteger,
  });

  return {
    ...params,
    setParams,
  };
}

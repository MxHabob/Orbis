"use server";

import { authActionClient } from "@/actions/safe-action";
import { getCurrentAcademicYear } from "@orbis/db/queries/academic-calendar";
import { getClassSchedule, getTeacherSchedule } from "@orbis/db/queries/lessons";
import { getStudentByUserId } from "@orbis/db/queries/students";
import { 
  addDays, 
  differenceInDays, 
  format, 
  parseISO, 
  getDay,
  parse
} from "date-fns";
import { z } from "zod";

export type CalendarEvent = {
  id: string;
  start: string;
  stop: string;
  duration: number;
  trackerProject: {
    name: string;
    color: string;
  };
  lessonId: number;
  subjectId: number;
  classId: number;
  description?: string;
  date?: string;
};

const getCalendarEventsSchema = z.object({
  from: z.string(),
  to: z.string(),
});

export const getCalendarEventsAction = authActionClient
  .schema(getCalendarEventsSchema)
  .metadata({
    name: "get-calendar-events",
  })
  .action(async ({ parsedInput: { from, to }, ctx: { user } }) => {
    const startDate = parseISO(from);
    const endDate = parseISO(to);

    // 1. Get Current Academic Year
    const currentYear = await getCurrentAcademicYear();
    
    if (!currentYear) {
        return {};
    }

    // 2. Determine Context (Student or Teacher)
    let lessons: any[] = [];

    if (user.role === 'student') {
        const studentProfile = await getStudentByUserId(user.id);
        if (studentProfile?.currentClassId) {
            lessons = await getClassSchedule(studentProfile.currentClassId, currentYear.id);
        }
    } else if (user.role === 'teacher') {
        lessons = await getTeacherSchedule(user.id, currentYear.id);
    } else {
        return {};
    }

    if (!lessons.length) {
        return {};
    }

    // 3. Expand Lessons into Events for the Date Range
    const result: Record<string, CalendarEvent[]> = {};
    const daysDiff = differenceInDays(endDate, startDate);

    for (let i = 0; i <= daysDiff; i++) {
        const currentDate = addDays(startDate, i);
        // getDay returns 0 (Sunday) to 6 (Saturday)
        const dayOfWeek = getDay(currentDate); 
        
        // Filter lessons for this day of week
        const dayLessons = lessons.filter(l => l.dayOfWeek === dayOfWeek);

        if (dayLessons.length > 0) {
            const dateKey = format(currentDate, "yyyy-MM-dd");
            result[dateKey] = dayLessons.map(lesson => {
                // Construct Start/End Times
                const startDateTime = parse(lesson.timeSlot.startTime, 'HH:mm:ss', currentDate);
                const endDateTime = parse(lesson.timeSlot.endTime, 'HH:mm:ss', currentDate);
                
                const startIso = format(startDateTime, "yyyy-MM-dd'T'HH:mm:ssXXX");
                const stopIso = format(endDateTime, "yyyy-MM-dd'T'HH:mm:ssXXX");
                
                // Determine project name based on context
                const projectName = user.role === 'student' 
                    ? `${lesson.subject.name} (${lesson.teacher?.name || 'Unknown'})`
                    : `${lesson.subject.name} - ${lesson.class?.name || 'Unknown'}`;

                const duration = (endDateTime.getTime() - startDateTime.getTime()) / 1000;

                return {
                    id: `${lesson.id}-${dateKey}`,
                    start: startIso,
                    stop: stopIso,
                    duration: duration > 0 ? duration : 2400, // 40 mins default if calc fails
                    trackerProject: {
                        name: projectName,
                        color: lesson.subject.color || '#000000',
                    },
                    lessonId: lesson.id,
                    subjectId: lesson.subject.id,
                    classId: lesson.class?.id || 0, // class might be missing in some views? NO, schedule query has it
                    date: dateKey,
                };
            });
        }
    }

    return result;
  });

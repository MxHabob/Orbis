"use client";

import { cn } from "@orbis/ui/cn";
import { Skeleton } from "@orbis/ui/skeleton";
import { useQuery } from "@tanstack/react-query";
import React from "react";
import { useLessonParams } from "@/hooks/use-lesson-params";
import { useTRPC } from "@/trpc/client";

const daysOfWeek = [
  { value: 0, label: "Sunday" },
  { value: 1, label: "Monday" },
  { value: 2, label: "Tuesday" },
  { value: 3, label: "Wednesday" },
  { value: 4, label: "Thursday" },
];

export function LessonSchedule() {
  const { classId, academicYearId, setParams } = useLessonParams();
  const trpc = useTRPC();

  // We need a yearId. We'll try to find the current one if not provided.
  const { data: academicYears } = useQuery(
    trpc.academic.getYears.queryOptions(),
  );
  const activeYearId =
    academicYearId || academicYears?.find((y) => y.isCurrent)?.id || 0;

  const { data: timeSlots, isLoading: isLoadingSlots } = useQuery(
    trpc.lessons.getTimeSlots.queryOptions(
      { yearId: activeYearId },
      { enabled: !!activeYearId },
    ),
  );

  const { data: lessons, isLoading: isLoadingLessons } = useQuery(
    trpc.lessons.getClassSchedule.queryOptions(
      { classId: classId!, yearId: activeYearId },
      { enabled: !!classId && !!activeYearId },
    ),
  );

  if (!classId) {
    return (
      <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
        <p>Please select a class to view its schedule.</p>
      </div>
    );
  }

  if (isLoadingSlots || isLoadingLessons) {
    return (
      <div className="grid grid-cols-6 gap-4 p-4">
        {Array.from({ length: 30 }).map((_, i) => (
          <Skeleton key={i} className="h-20 w-full rounded-xl" />
        ))}
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="overflow-auto flex-1">
        <table className="w-full border-collapse">
          <thead>
            <tr>
              <th className="p-4 border bg-muted/50 text-left font-medium w-32">
                Period
              </th>
              {daysOfWeek.map((day) => (
                <th
                  key={day.value}
                  className="p-4 border bg-muted/50 text-left font-medium min-w-[150px]"
                >
                  {day.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {timeSlots?.map((slot) => (
              <tr key={slot.id}>
                <td className="p-4 border font-medium">
                  <div className="text-sm">{slot.slotNumber}</div>
                  <div className="text-xs text-muted-foreground">
                    {slot.startTime.substring(0, 5)} -{" "}
                    {slot.endTime.substring(0, 5)}
                  </div>
                </td>
                {daysOfWeek.map((day) => {
                  const lesson = lessons?.find(
                    (l) =>
                      l.timeSlotId === slot.id && l.dayOfWeek === day.value,
                  );

                  return (
                    <td
                      key={day.value}
                      className={cn(
                        "p-2 border h-24 hover:bg-muted/30 transition-colors cursor-pointer relative",
                        !lesson && "bg-muted/5",
                      )}
                      onClick={() => {
                        if (lesson) {
                          setParams({ update: true, lessonId: lesson.id });
                        } else {
                          setParams({
                            create: true,
                            dayOfWeek: day.value,
                            timeSlotId: slot.id,
                            classId: classId,
                          });
                        }
                      }}
                    >
                      {lesson ? (
                        <div className="flex flex-col h-full bg-primary/10 rounded-lg p-2 border-l-4 border-primary shadow-sm">
                          <span className="text-sm font-semibold truncate">
                            {lesson.subject?.name}
                          </span>
                          <span className="text-xs text-muted-foreground truncate mt-1">
                            {lesson.teacher?.name}
                          </span>
                          <span className="text-[10px] absolute bottom-2 right-2 opacity-50">
                            {lesson.room?.name || "No Room"}
                          </span>
                        </div>
                      ) : (
                        <div className="opacity-0 hover:opacity-100 flex items-center justify-center h-full text-xs text-muted-foreground border-2 border-dashed rounded-lg">
                          + Add Lesson
                        </div>
                      )}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

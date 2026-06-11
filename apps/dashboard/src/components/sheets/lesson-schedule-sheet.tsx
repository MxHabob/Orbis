"use client";

import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import React from "react";
import { useLessonParams } from "@/hooks/use-lesson-params";
import { LessonSchedule } from "../lesson-schedule";

export function LessonScheduleSheet() {
  const { setParams, schedule, classId } = useLessonParams();

  const isOpen = Boolean(schedule);

  return (
    <Sheet
      open={isOpen}
      onOpenChange={() =>
        setParams({
          schedule: null,
          classId: null,
          timeSlotId: null,
          dayOfWeek: null,
        })
      }
    >
      <SheetContent className="sm:max-w-[90vw] lg:max-w-[80vw]">
        <SheetHeader className="mb-8">
          <h2 className="text-xl">School Schedule Management</h2>
        </SheetHeader>
        <LessonSchedule />
      </SheetContent>
    </Sheet>
  );
}

"use client";

import { ScrollArea } from "@orbis/ui/scroll-area";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import React from "react";
import { LessonForm } from "@/components/forms/lesson-form";
import { useLessonParams } from "@/hooks/use-lesson-params";

export function LessonCreateSheet() {
  const { setParams, create } = useLessonParams();

  const isOpen = Boolean(create);

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams({ create: null })}>
      <SheetContent>
        <SheetHeader className="mb-8 flex justify-between items-center flex-row">
          <h2 className="text-xl">Create New Lesson</h2>
        </SheetHeader>

        <ScrollArea className="h-full p-0 pb-28" hideScrollbar>
          <LessonForm />
        </ScrollArea>
      </SheetContent>
    </Sheet>
  );
}

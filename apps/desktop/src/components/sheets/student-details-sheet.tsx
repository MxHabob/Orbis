"use client";

import { Sheet, SheetContent } from "@orbis/ui/sheet";
import { useQuery } from "@tanstack/react-query";
import React from "react";
import { useStudentParams } from "@/hooks/use-student-params";
import { useTRPC } from "@/trpc/client";
import { StudentForm } from "../forms/student-form";

export function StudentDetailsSheet() {
  const trpc = useTRPC();
  const { studentId, details, setParams } = useStudentParams();

  const isOpen = Boolean(studentId && details);

  const { data: student } = useQuery({
    ...trpc.students.getById.queryOptions({ id: studentId! }),
    enabled: isOpen,
  });

  return (
    <Sheet
      open={isOpen}
      onOpenChange={() => setParams({ studentId: null, details: null })}
    >
      <SheetContent style={{ maxWidth: 620 }} className="pb-4">
        <div className="mt-6">
          {/* For now we reuse StudentForm, we can create a dedicated details view later */}
          <StudentForm data={student} />
        </div>
      </SheetContent>
    </Sheet>
  );
}

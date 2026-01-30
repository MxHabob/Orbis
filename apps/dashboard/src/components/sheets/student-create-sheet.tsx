"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@midday/ui/sheet";
import React from "react";
import { StudentForm } from "../forms/student-form";

export function StudentCreateSheet() {
  const { setParams, createStudent } = useStudentParams();

  const isOpen = Boolean(createStudent);

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent stack>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Add Student</h2>
          <Button
            size="icon"
            variant="ghost"
            onClick={() => setParams(null)}
            className="p-0 m-0 size-auto hover:bg-transparent"
          >
            <Icons.Close className="size-5" />
          </Button>
        </SheetHeader>

        <StudentForm />
      </SheetContent>
    </Sheet>
  );
}

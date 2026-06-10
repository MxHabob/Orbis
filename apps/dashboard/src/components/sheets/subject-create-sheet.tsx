"use client";

import { useSubjectParams } from "@/hooks/use-subject-params";
import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import React from "react";
import { SubjectForm } from "../forms/subject-form";

export function SubjectCreateSheet() {
  const { setParams, createSubject } = useSubjectParams();

  const isOpen = Boolean(createSubject);

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Create Subject</h2>
          <Button
            size="icon"
            variant="ghost"
            onClick={() => setParams(null)}
            className="p-0 m-0 size-auto hover:bg-transparent"
          >
            <Icons.Close className="size-5" />
          </Button>
        </SheetHeader>

        <SubjectForm />
      </SheetContent>
    </Sheet>
  );
}

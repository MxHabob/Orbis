"use client";

import { useClassParams } from "@/hooks/use-class-params";
import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import React from "react";
import { ClassForm } from "../forms/class-form";

export function ClassCreateSheet() {
  const { setParams, createClass } = useClassParams();

  const isOpen = Boolean(createClass);

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Create Class</h2>
          <Button
            size="icon"
            variant="ghost"
            onClick={() => setParams(null)}
            className="p-0 m-0 size-auto hover:bg-transparent"
          >
            <Icons.Close className="size-5" />
          </Button>
        </SheetHeader>

        <ClassForm />
      </SheetContent>
    </Sheet>
  );
}

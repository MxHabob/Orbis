"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";

export function OpenStudentSheet() {
  const { setParams } = useStudentParams();

  return (
    <div>
      <Button
        variant="outline"
        size="icon"
        onClick={() => setParams({ createStudent: true})}
      >
        <Icons.Add />
      </Button>
    </div>
  );
}

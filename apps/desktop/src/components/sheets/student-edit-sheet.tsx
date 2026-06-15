"use client";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useStudentParams } from "@/hooks/use-student-params";
import { useTRPC } from "@/ipc/query-adapter";
import { StudentForm } from "../forms/student-form";

export function StudentEditSheet() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, studentId, details } = useStudentParams();

  const isOpen = Boolean(studentId && !details);

  const { data: student } = useQuery(
    trpc.students.getById.queryOptions(
      { id: studentId! },
      {
        enabled: isOpen,
        staleTime: 0,
        initialData: () => {
          const pages = queryClient
            .getQueriesData({ queryKey: trpc.students.getAll.queryKey() })
            // @ts-expect-error
            .flatMap(([, data]) => data?.pages ?? [])
            .flatMap((page) => page.data ?? []);

          return pages.find((d) => d.userId === studentId);
        },
      },
    ),
  );

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent stack>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Edit Student</h2>

          {studentId && (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button type="button">
                  <Icons.MoreVertical className="size-5" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent sideOffset={10} align="end">
                <DropdownMenuItem
                  className="text-destructive"
                  onSelect={() => setParams({ deleteStudentId: studentId })}
                >
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </SheetHeader>

        <StudentForm data={student} key={student?.userId} />
      </SheetContent>
    </Sheet>
  );
}

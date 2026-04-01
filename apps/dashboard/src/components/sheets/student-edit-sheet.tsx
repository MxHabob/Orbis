"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { useTRPC } from "@/trpc/client";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@midday/ui/alert-dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { Icons } from "@midday/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@midday/ui/sheet";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { StudentForm } from "../forms/student-form";
import { invalidateStudentDomain } from "@/utils/query-invalidation";

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

  const deleteStudentMutation = useMutation(
    trpc.students.delete.mutationOptions({
      onSuccess: () => {
        void invalidateStudentDomain(queryClient, trpc);
        setParams(null);
      },
    }),
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
                <AlertDialog>
                  <AlertDialogTrigger asChild>
                    <DropdownMenuItem
                      className="text-destructive"
                      onSelect={(e) => e.preventDefault()}
                    >
                      Delete
                    </DropdownMenuItem>
                  </AlertDialogTrigger>
                  <AlertDialogContent>
                    <AlertDialogHeader>
                      <AlertDialogTitle>
                        Are you absolutely sure?
                      </AlertDialogTitle>
                      <AlertDialogDescription>
                        This action cannot be undone. This will permanently
                        delete the student and remove their data from our
                        servers.
                      </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                      <AlertDialogCancel>Cancel</AlertDialogCancel>
                      <AlertDialogAction
                        onClick={() =>
                          deleteStudentMutation.mutate({ userId: studentId })
                        }
                      >
                        Delete
                      </AlertDialogAction>
                    </AlertDialogFooter>
                  </AlertDialogContent>
                </AlertDialog>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </SheetHeader>

        <StudentForm data={student} key={student?.userId} />
      </SheetContent>
    </Sheet>
  );
}

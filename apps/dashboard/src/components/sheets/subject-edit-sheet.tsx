"use client";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useSubjectParams } from "@/hooks/use-subject-params";
import { useTRPC } from "@/trpc/client";
import { SubjectEditForm } from "../forms/subject-edit-form";

export function SubjectEditSheet() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, subjectId } = useSubjectParams();

  const isOpen = Boolean(subjectId);

  const { data: subjectData } = useQuery(
    trpc.subjects.getById.queryOptions(
      { id: Number(subjectId) },
      {
        enabled: isOpen,
      },
    ),
  );

  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Edit Subject</h2>

          {subjectId && (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button type="button">
                  <Icons.MoreVertical className="size-5" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent sideOffset={10} align="end">
                <DropdownMenuItem
                  className="text-destructive"
                  onSelect={() => setParams({ deleteSubjectId: subjectId })}
                >
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </SheetHeader>

        <SubjectEditForm data={subjectData} key={subjectData?.id} />
      </SheetContent>
    </Sheet>
  );
}

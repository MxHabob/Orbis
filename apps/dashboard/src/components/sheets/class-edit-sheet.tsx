"use client";

import { useClassParams } from "@/hooks/use-class-params";
import { useTRPC } from "@/trpc/client";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ClassEditForm } from "../forms/class-edit-form";

export function ClassEditSheet() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, classId } = useClassParams();

  const isOpen = Boolean(classId);

  const { data: classData } = useQuery(
    trpc.classes.getById.queryOptions(
      { id: Number(classId) },
      {
        enabled: isOpen,
      },
    ),
  );



  return (
    <Sheet open={isOpen} onOpenChange={() => setParams(null)}>
      <SheetContent>
        <SheetHeader className="mb-6 flex justify-between items-center flex-row">
          <h2 className="text-xl">Edit Class</h2>

          {classId && (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button type="button">
                  <Icons.MoreVertical className="size-5" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent sideOffset={10} align="end">
                <DropdownMenuItem
                  className="text-destructive"
                  onSelect={() => setParams({ deleteClassId: classId })}
                >
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </SheetHeader>

        <ClassEditForm data={classData} key={classData?.id} />
      </SheetContent>
    </Sheet>
  );
}

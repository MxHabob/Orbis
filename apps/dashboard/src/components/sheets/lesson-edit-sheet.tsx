"use client";

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
} from "@orbis/ui/alert-dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { ScrollArea } from "@orbis/ui/scroll-area";
import { Sheet, SheetContent, SheetHeader } from "@orbis/ui/sheet";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import React from "react";
import { LessonForm } from "@/components/forms/lesson-form";
import { useLessonParams } from "@/hooks/use-lesson-params";
import { useTRPC } from "@/trpc/client";

export function LessonEditSheet() {
  const { setParams, update, lessonId } = useLessonParams();
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  const isOpen = update !== null && Boolean(lessonId);

  const { data } = useQuery(
    trpc.lessons.getById.queryOptions(
      { id: lessonId! },
      {
        enabled: isOpen,
      },
    ),
  );

  const deleteMutation = useMutation(
    trpc.lessons.delete.mutationOptions({
      onSuccess: () => {
        setParams({ lessonId: null, update: null });
        queryClient.invalidateQueries({
          queryKey: trpc.lessons.getClassSchedule.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.lessons.getTeacherSchedule.queryKey(),
        });
      },
    }),
  );

  return (
    <AlertDialog>
      <Sheet
        open={isOpen}
        onOpenChange={() => setParams({ update: null, lessonId: null })}
      >
        <SheetContent>
          <SheetHeader className="mb-8 flex justify-between items-center flex-row">
            <h2 className="text-xl">Edit Lesson</h2>

            <DropdownMenu>
              <DropdownMenuTrigger>
                <Icons.MoreVertical className="w-5 h-5" />
              </DropdownMenuTrigger>

              <DropdownMenuContent className="w-42" sideOffset={10} align="end">
                <AlertDialogTrigger asChild>
                  <DropdownMenuItem className="text-destructive">
                    Delete
                  </DropdownMenuItem>
                </AlertDialogTrigger>
              </DropdownMenuContent>
            </DropdownMenu>
          </SheetHeader>

          <ScrollArea className="h-full p-0 pb-28" hideScrollbar>
            <LessonForm data={data} />
          </ScrollArea>
        </SheetContent>
      </Sheet>

      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            This action cannot be undone. This will permanently delete this
            lesson from the schedule.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction
            onClick={() => deleteMutation.mutate({ id: lessonId! })}
          >
            Delete
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

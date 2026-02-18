import { useI18n, useScopedI18n } from "@/locales/client";
import type { RouterOutputs } from "@api/trpc/routers/_app";
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
import { Avatar, AvatarFallback, AvatarImageNext } from "@midday/ui/avatar";
import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useToast } from "@midday/ui/use-toast";
import type { ColumnDef, FilterFn, Row } from "@tanstack/react-table";
import { MoreHorizontal } from "lucide-react";
import { Loader2 } from "lucide-react";
import * as React from "react";
import "@tanstack/react-table";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/navigation";

type TeamMember = RouterOutputs["user"]["getStaff"][number];

const userFilterFn: FilterFn<TeamMember> = (
  row: Row<TeamMember>,
  _: string,
  filterValue: string,
) => {
  const memberName = row.original.name?.toLowerCase();

  return memberName?.includes(filterValue.toLowerCase()) ?? false;
};

export const columns: ColumnDef<TeamMember>[] = [
  {
    id: "user",
    accessorKey: "name",
    filterFn: userFilterFn,
    cell: ({ row }) => {
      return (
        <div>
          <div className="flex items-center space-x-4">
            <Avatar className="rounded-full w-8 h-8">
              <AvatarImageNext
                src={row.original.image ?? ""}
                alt={row.original.name ?? ""}
                width={32}
                height={32}
              />
              <AvatarFallback>
                <span className="text-xs">
                  {row.original.name?.charAt(0)?.toUpperCase()}
                </span>
              </AvatarFallback>
            </Avatar>
            <div className="flex flex-col">
              <span className="font-medium text-sm">
                {row.original.name}
              </span>
              <span className="text-sm text-[#606060]">
                {row.original.email}
              </span>
            </div>
          </div>
        </div>
      );
    },
  },
  {
    id: "actions",
    cell: ({ row, table }) => {
      const t = useI18n();
      const tRoles = useScopedI18n("roles");
      const { toast } = useToast();
      const meta = table.options.meta;
      const trpc = useTRPC();
      const queryClient = useQueryClient();
      const router = useRouter();

      const deleteMemberMutation = useMutation(
        trpc.user.delete.mutationOptions({
          onSuccess: () => {
            queryClient.invalidateQueries({
              queryKey: trpc.user.getStaff.queryKey(),
            });
          },
          onError: () => {
            toast({
              title: "Error deleting member",
            });
          },
        }),
      );

      const leaveTeamMutation = useMutation(
        trpc.user.delete.mutationOptions({
          onSuccess: async () => {
            router.push("/teams");
          },
        }),
      );

      const updateMemberMutation = useMutation(
        trpc.user.update.mutationOptions({
          onSuccess: () => {
            queryClient.invalidateQueries({
              queryKey: trpc.user.getStaff.queryKey(),
            });
          },
        }),
      );

      return (
        <div className="flex justify-end">
          <div className="flex space-x-2 items-center">
            {(meta?.currentUser?.role === "admin" &&
              meta?.currentUser?.id !== row.original.id) ||
            (meta?.currentUser?.role === "admin" &&
              (meta?.totalOwners ?? 0) > 1) ? (
              <Select
                value={row.original.role ?? undefined}
                onValueChange={(role) => {
                  updateMemberMutation.mutate({
                    id: row.original.id!,
                    role: role as "admin" | "student" | "parent" | "teacher" | "staff",
                  });
                }}
              >
                <SelectTrigger>
                  <SelectValue
                    placeholder={tRoles((row.original.role as any) || "member")}
                  />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="admin">Admin</SelectItem>
                  <SelectItem value="student">Student</SelectItem>
                  <SelectItem value="parent">Parent</SelectItem>
                  <SelectItem value="teacher">Teacher</SelectItem>
                  <SelectItem value="staff">Staff</SelectItem>
                </SelectContent>
              </Select>
            ) : (
              <span className="text-sm text-[#606060]">
                {tRoles((row.original.role as any) || "member")}
              </span>
            )}
            {meta?.currentUser?.role === "admin" && (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" className="h-8 w-8 p-0">
                    <MoreHorizontal className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  {meta?.currentUser?.id !== row.original.id && (
                    <AlertDialog>
                      <DropdownMenuItem
                        className="text-destructive"
                        asDialogTrigger
                      >
                        <AlertDialogTrigger>Remove Member</AlertDialogTrigger>
                      </DropdownMenuItem>

                      <AlertDialogContent>
                        <AlertDialogHeader>
                          <AlertDialogTitle>
                            Remove Team Member
                          </AlertDialogTitle>
                          <AlertDialogDescription>
                            You are about to remove the following Team Member,
                            are you sure you want to continue?
                          </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                          <AlertDialogCancel>Cancel</AlertDialogCancel>
                          <AlertDialogAction
                            disabled={deleteMemberMutation.isPending}
                            onClick={() => {
                              deleteMemberMutation.mutate({
                                id: row.original.id!,
                              });
                            }}
                          >
                            {deleteMemberMutation.isPending ? (
                              <Loader2 className="h-4 w-4 animate-spin" />
                            ) : (
                              "Confirm"
                            )}
                          </AlertDialogAction>
                        </AlertDialogFooter>
                      </AlertDialogContent>
                    </AlertDialog>
                  )}

                  {meta?.currentUser?.id === row.original.id && (
                    <AlertDialog>
                      <DropdownMenuItem
                        className="text-destructive"
                        asDialogTrigger
                      >
                        <AlertDialogTrigger>Leave Team</AlertDialogTrigger>
                      </DropdownMenuItem>

                      <AlertDialogContent>
                        <AlertDialogHeader>
                          <AlertDialogTitle>Leave Team</AlertDialogTitle>
                          <AlertDialogDescription>
                            You are about to leave this team. In order to regain
                            access at a later time, a Team Owner must invite
                            you.
                            <p className="mt-4">
                              Are you sure you want to continue?
                            </p>
                          </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                          <AlertDialogCancel>Cancel</AlertDialogCancel>
                          <AlertDialogAction
                            disabled={leaveTeamMutation.isPending}
                            onClick={() =>
                              leaveTeamMutation.mutate({
                                id: row.original.id!,
                              })
                            }
                          >
                            {leaveTeamMutation.isPending ? (
                              <Loader2 className="h-4 w-4 animate-spin" />
                            ) : (
                              "Confirm"
                            )}
                          </AlertDialogAction>
                        </AlertDialogFooter>
                      </AlertDialogContent>
                    </AlertDialog>
                  )}
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </div>
        </div>
      );
    },
    meta: {
      className: "text-right",
    },
  },
];

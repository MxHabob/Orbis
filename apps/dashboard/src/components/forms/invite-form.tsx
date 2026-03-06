"use client";

import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import { Button } from "@midday/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel } from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { SubmitButton } from "@midday/ui/submit-button";
import { useToast } from "@midday/ui/use-toast";
import MultipleSelector, { type Option } from "@midday/ui/multiple-selector";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import Link from "next/link";
import { useFieldArray, useWatch } from "react-hook-form";
import { z } from "zod/v3";
import { X } from "lucide-react";

const formSchema = z.object({
  invites: z.array(
    z.object({
      name: z.string().min(2, "Name is required"),
      email: z.string().email(),
      role: z.enum(["admin", "teacher", "staff", "parent", "student"]),
      studentIds: z.array(z.string()).default([]),
      relationship: z.enum(["parent", "mother", "father", "guardian"]).default("parent"),
    }),
  ),
});

type InviteFormProps = {
  onSuccess?: () => void;
  skippable?: boolean;
};

export function InviteForm({ onSuccess, skippable = true }: InviteFormProps) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();

  // Fetch all students for the parent-student linking
  const studentsQuery = useQuery(
    trpc.students.getAll.queryOptions()
  );

  const studentOptions: Option[] = (studentsQuery.data ?? []).map((s) => ({
    value: s.userId,
    label: `${s.name ?? "Unknown"} — ${s.studentNumber}`,
    studentNumber: s.studentNumber,
  }));

  const inviteMutation = useMutation(
    trpc.user.create.mutationOptions({
      onSuccess: (data) => {
        queryClient.invalidateQueries({
          queryKey: trpc.user.get.queryKey(),
        });

        if (data && data.length > 0) {
          toast({
            title: "Invites sent",
            description: `Successfully created ${data.length} account(s).`,
            variant: "success",
          });
        }

        onSuccess?.();
      },
      onError: (error) => {
        toast({
          title: "Error",
          description: error.message || "Failed to create accounts.",
          variant: "destructive",
        });
      },
    }),
  );

  const form = useZodForm(formSchema, {
    defaultValues: {
      invites: [
        {
          name: "",
          email: "",
          role: "teacher",
          studentIds: [],
          relationship: "parent",
        },
      ],
    },
  });

  const onSubmit = form.handleSubmit((data) => {
    console.log("Submitting invites:", data);
    inviteMutation.mutate(
      data.invites
        .filter((invite) => invite.email !== "")
        .map((invite) => ({
          ...invite,
          studentIds: invite.role === "parent" ? invite.studentIds : undefined,
          relationship: invite.role === "parent" ? invite.relationship : undefined,
        })),
    );
  });

  const { fields, append, remove } = useFieldArray({
    name: "invites",
    control: form.control,
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <div className="space-y-5">
          {fields.map((field, index) => (
            <InviteRow
              key={field.id}
              index={index}
              control={form.control}
              studentOptions={studentOptions}
              isLoadingStudents={studentsQuery.isLoading}
              onRemove={fields.length > 1 ? () => remove(index) : undefined}
            />
          ))}
        </div>

        <Button
          variant="outline"
          type="button"
          className="mt-4 border-none bg-[#F2F1EF] text-[11px] dark:bg-[#1D1D1D]"
          onClick={() =>
            append({
              name: "",
              email: "",
              role: "teacher",
              studentIds: [],
              relationship: "parent",
            })
          }
        >
          Add more
        </Button>

        <div className="border-t-[1px] pt-4 mt-4 flex items-center justify-between">
          <div>
            {Object.values(form.formState.errors).length > 0 && (
              <span className="text-sm text-destructive">
                Please complete the fields above.
              </span>
            )}
          </div>

          <div className="flex items-center justify-between">
            {skippable ? (
              <Link href="/">
                <Button
                  variant="ghost"
                  className="p-0 hover:bg-transparent font-normal"
                >
                  Skip this step
                </Button>
              </Link>
            ) : (
              <div />
            )}

            <SubmitButton
              type="submit"
              isSubmitting={inviteMutation.isPending}
              disabled={inviteMutation.isPending}
            >
              Create accounts
            </SubmitButton>
          </div>
        </div>
      </form>
    </Form>
  );
}

// ==========================================
// Individual Invite Row Component
// ==========================================
function InviteRow({
  index,
  control,
  studentOptions,
  isLoadingStudents,
  onRemove,
}: {
  index: number;
  control: any;
  studentOptions: Option[];
  isLoadingStudents: boolean;
  onRemove?: () => void;
}) {
  const selectedRole = useWatch({
    control,
    name: `invites.${index}.role`,
  });

  const isParent = selectedRole === "parent";

  return (
    <div className="relative rounded-lg px-1 py-1 transition-all duration-200">
      {/* Remove button */}
      {onRemove && (
        <button
          type="button"
          onClick={onRemove}
          className="absolute -right-2 -top-2 rounded-full bg-background border border-border p-1 text-muted-foreground hover:text-destructive hover:border-destructive transition-colors z-10"
        >
          <X className="size-3" />
        </button>
      )}

      <div className="grid grid-cols-12 gap-3">
        <FormField
          control={control}
          name={`invites.${index}.name`}
          render={({ field }) => (
            <FormItem className="col-span-4">
              <FormControl>
                <Input
                  placeholder="Full Name"
                  autoComplete="off"
                  {...field}
                />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={control}
          name={`invites.${index}.email`}
          render={({ field }) => (
            <FormItem className="col-span-5">
              <FormControl>
                <Input
                  placeholder="jane@example.com"
                  type="email"
                  autoComplete="off"
                  autoCapitalize="none"
                  autoCorrect="off"
                  spellCheck="false"
                  {...field}
                />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={control}
          name={`invites.${index}.role`}
          render={({ field }) => (
            <FormItem className="col-span-3">
              <Select
                onValueChange={field.onChange}
                defaultValue={field.value}
              >
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Role" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="admin">Admin</SelectItem>
                  <SelectItem value="teacher">Teacher</SelectItem>
                  <SelectItem value="staff">Staff</SelectItem>
                  <SelectItem value="parent">Parent</SelectItem>
                  <SelectItem value="student">Student</SelectItem>
                </SelectContent>
              </Select>
            </FormItem>
          )}
        />
      </div>

      {isParent && (
        <div className="mt-4 pt-4 border-t border-dashed border-border/50 animate-in slide-in-from-top-2 fade-in-0 duration-200">
          <div className="grid grid-cols-12 gap-3">
            <FormField
              control={control}
              name={`invites.${index}.relationship`}
              render={({ field }) => (
                <FormItem className="col-span-3">
                  <FormLabel className="text-xs font-medium text-muted-foreground">
                    Relationship
                  </FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Relationship" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="parent">Parent</SelectItem>
                      <SelectItem value="mother">Mother</SelectItem>
                      <SelectItem value="father">Father</SelectItem>
                      <SelectItem value="guardian">Guardian</SelectItem>
                    </SelectContent>
                  </Select>
                </FormItem>
              )}
            />
            <FormField
              control={control}
              name={`invites.${index}.studentIds`}
              render={({ field }) => (
                <FormItem className="col-span-9">
                  <FormLabel className="text-xs font-medium text-muted-foreground">
                    Link to Students
                  </FormLabel>
                  <FormControl>
                    <MultipleSelector
                      value={
                        field.value?.map((id: string) => {
                          const opt = studentOptions.find(
                            (o) => o.value === id,
                          );
                          return opt ?? { value: id, label: id };
                        }) ?? []
                      }
                      onChange={(options) =>
                        field.onChange(options.map((o) => o.value))
                      }
                      defaultOptions={studentOptions}
                      placeholder={
                        isLoadingStudents
                          ? "Loading students..."
                          : "Search students by name or number..."
                      }
                      disabled={isLoadingStudents}
                      emptyIndicator={
                        <p className="text-center text-sm text-muted-foreground">
                          No students found.
                        </p>
                      }
                      className="border rounded-md"
                      hidePlaceholderWhenSelected
                    />
                  </FormControl>
                </FormItem>
              )}
            />
          </div>
        </div>
      )}
    </div>
  );
}

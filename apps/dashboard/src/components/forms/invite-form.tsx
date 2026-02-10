"use client";

import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import { Button } from "@midday/ui/button";
import { Form, FormControl, FormField, FormItem } from "@midday/ui/form";
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
import { useMutation, useQueryClient } from "@tanstack/react-query";
import Link from "next/link";
import { useFieldArray } from "react-hook-form";
import { z } from "zod/v3";

const formSchema = z.object({
  invites: z.array(
    z.object({
      name: z.string().min(2, "Name is required"),
      email: z.string().email(),
      role: z.enum(["admin", "teacher", "staff", "parent", "student"]),
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

  const inviteMutation = useMutation(
    trpc.user.create.mutationOptions({
      onSuccess: (data) => {
        queryClient.invalidateQueries({
          queryKey: trpc.user.get.queryKey(),
        });

        // Show appropriate feedback based on results
        if (data && data.length > 0) {
          toast({
            title: "Invites sent",
            description: `Successfully sent ${data.length} invite(s).`,
            variant: "success",
          });
        }

        onSuccess?.();
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
        },
      ],
    },
  });

  const onSubmit = form.handleSubmit((data) => {
    inviteMutation.mutate(data.invites.filter((invite) => invite.email !== ""));
  });

  const { fields, append } = useFieldArray({
    name: "invites",
    control: form.control,
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        {fields.length > 0 && (
          <div className="grid grid-cols-12 gap-4 mb-2 px-1 text-xs font-medium text-muted-foreground uppercase tracking-wider">
            <div className="col-span-4">Full Name</div>
            <div className="col-span-5">Email Address</div>
            <div className="col-span-3">Role</div>
          </div>
        )}
        {fields.map((field, index) => (
          <div
            className="grid grid-cols-12 gap-4 mt-4 items-start border-b pb-4 last:border-0"
            key={field.id}
          >
            <FormField
              control={form.control}
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
              control={form.control}
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
              control={form.control}
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
        ))}

        <Button
          variant="outline"
          type="button"
          className="mt-4 border-none bg-[#F2F1EF] text-[11px] dark:bg-[#1D1D1D]"
          onClick={() => append({ name: "", email: "", role: "teacher" })}
        >
          Add more
        </Button>

        <div className="border-t-[1px] pt-4 mt-8 items-center justify-between">
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

"use client";


import { useZodForm } from "@/hooks/use-zod-form";
import { authClient } from "@midday/auth/auth-client";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@midday/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { SubmitButton } from "@midday/ui/submit-button";
import { z } from "zod/v3";

const formSchema = z.object({
  name: z.string().min(2).max(32),
});

export function CompanyName() {
  const { data, isPending } = authClient.useSession();

  const form = useZodForm(formSchema, {
    defaultValues: {
      name: data?.user?.name ?? "",
    },
  });

  const onSubmit = form.handleSubmit((data) => {
    // updateTeamMutation.mutate({ fullName: data.name });
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <Card>
          <CardHeader>
            <CardTitle>User name</CardTitle>
            <CardDescription>
              This is your user's visible name within Midday. For example,
              the name of your user or department.
            </CardDescription>
          </CardHeader>

          <CardContent>
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input
                      {...field}
                      className="max-w-[300px]"
                      autoComplete="off"
                      autoCapitalize="none"
                      autoCorrect="off"
                      spellCheck="false"
                      maxLength={32}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </CardContent>

          <CardFooter className="flex justify-between">
            <div>Please use 32 characters at maximum.</div>
            <SubmitButton
              isSubmitting={isPending}
              disabled={isPending}
            >
              Save
            </SubmitButton>
          </CardFooter>
        </Card>
      </form>
    </Form>
  );
}

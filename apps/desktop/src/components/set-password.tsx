"use client";

import { authClient } from "@orbis/auth/auth-client";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import { SubmitButton } from "@orbis/ui/submit-button";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { z } from "zod/v3";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";

const formSchema = z
  .object({
    password: z.string().min(8, "Password must be at least 8 characters"),
    confirmPassword: z.string().min(8, "Please confirm your password"),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

export function SetPassword() {
  const { toast } = useToast();
  const searchParams = useSearchParams();
  const isSetup = searchParams.get("setup") === "true";

  const form = useZodForm(formSchema, {
    defaultValues: {
      password: "",
      confirmPassword: "",
    },
  });

  const trpc = useTRPC();
  const updatePasswordMutation = useMutation(
    trpc.user.updatePassword.mutationOptions({
      onSuccess: () => {
        toast({
          title: "Success",
          description:
            "Password set successfully. Your account is now fully active.",
        });
        // Clear setup param if present
        if (isSetup) {
          window.history.replaceState({}, "", "/account");
        }
        form.reset();
      },
      onError: (error) => {
        toast({
          title: "Error",
          description: error.message || "Failed to set password",
          variant: "destructive",
        });
      },
    }),
  );

  const onSubmit = form.handleSubmit(async (data) => {
    updatePasswordMutation.mutate({
      password: data.password,
    });
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <Card
          className={isSetup ? "border-primary border-2 animate-pulse" : ""}
        >
          <CardHeader>
            <CardTitle>
              {isSetup ? "Complete Your Account Setup" : "Change Password"}
            </CardTitle>
            <CardDescription>
              {isSetup
                ? "Please set a password for your account to finish the setup process."
                : "Enter a new password below."}
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input
                      {...field}
                      type="password"
                      placeholder="New Password"
                      className="max-w-[300px]"
                      autoComplete="new-password"
                      disabled={updatePasswordMutation.isPending}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="confirmPassword"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input
                      {...field}
                      type="password"
                      placeholder="Confirm New Password"
                      className="max-w-[300px]"
                      autoComplete="new-password"
                      disabled={updatePasswordMutation.isPending}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </CardContent>

          <CardFooter className="flex justify-between">
            <div className="text-xs text-muted-foreground">
              Minimum 8 characters
            </div>
            <SubmitButton
              type="submit"
              disabled={updatePasswordMutation.isPending}
              isSubmitting={updatePasswordMutation.isPending}
            >
              {isSetup ? "Complete Setup" : "Change Password"}
            </SubmitButton>
          </CardFooter>
        </Card>
      </form>
    </Form>
  );
}

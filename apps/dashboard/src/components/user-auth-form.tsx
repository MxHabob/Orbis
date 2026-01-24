"use client";

import { authClient } from "@midday/auth/auth-client";
import { cn } from "@midday/ui/cn";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { SubmitButton } from "@midday/ui/submit-button";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { toast } from "@midday/ui/use-toast";

const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8, "Password must be at least 8 characters"),
  name: z.string().min(2, "Name must be at least 2 characters").optional(),
});

type Props = {
  className?: string;
  mode?: "sign-in" | "sign-up";
};

export function UserAuthForm({ className, mode = "sign-in" }: Props) {
  const [isLoading, setIsLoading] = useState(false);
  const [authMode, setAuthMode] = useState<"sign-in" | "sign-up">(mode);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
      name: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsLoading(true);

    try {
      if (authMode === "sign-up") {
        const { error } = await authClient.signUp.email({
          email: values.email,
          password: values.password,
          name: values.name || values.email.split("@")[0],
          callbackURL: "/",
        });

        if (error) {
          toast({
            title: "Error",
            description: error.message || "Failed to sign up",
            variant: "destructive",
          });
        } else {
          toast({
            title: "Success",
            description: "Account created successfully",
          });
          window.location.href = "/";
        }
      } else {
        const { error } = await authClient.signIn.email({
          email: values.email,
          password: values.password,
          callbackURL: "/",
        });

        if (error) {
          toast({
            title: "Error",
            description: error.message || "Invalid email or password",
            variant: "destructive",
          });
        } else {
          window.location.href = "/";
        }
      }
    } catch (error) {
      toast({
        title: "Error",
        description: "An unexpected error occurred",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <div className={cn("w-full", className)}>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          {authMode === "sign-up" && (
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input
                      placeholder="Full Name"
                      {...field}
                      autoCapitalize="words"
                      autoCorrect="off"
                      spellCheck="false"
                      disabled={isLoading}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input
                    placeholder="Email address"
                    {...field}
                    autoCapitalize="none"
                    autoComplete="email"
                    autoCorrect="off"
                    spellCheck="false"
                    disabled={isLoading}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="Password"
                    {...field}
                    autoComplete={authMode === "sign-up" ? "new-password" : "current-password"}
                    disabled={isLoading}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <SubmitButton
            type="submit"
            className="w-full bg-primary text-secondary font-medium h-[40px]"
            isSubmitting={isLoading}
          >
            {authMode === "sign-in" ? "Sign In" : "Create Account"}
          </SubmitButton>
        </form>
      </Form>

      <div className="mt-4 text-center text-sm">
        <button
          type="button"
          onClick={() => setAuthMode(authMode === "sign-in" ? "sign-up" : "sign-in")}
          className="text-primary hover:underline"
        >
          {authMode === "sign-in"
            ? "Don't have an account? Sign up"
            : "Already have an account? Sign in"}
        </button>
      </div>
    </div>
  );
}

"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { authClient } from "@/lib/auth-client";
import { cn } from "@orbis/ui/cn";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import { SubmitButton } from "@orbis/ui/submit-button";
import { toast } from "@orbis/ui/use-toast";
import { Loader2 } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useTRPC } from "@/ipc/query-adapter";

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
  const [accountStatus, setAccountStatus] = useState<
    "absent" | "exists" | "needs_password" | null
  >(null);
  const [isCheckingStatus, setIsCheckingStatus] = useState(false);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const trpc = useTRPC();

  async function handleEmailBlur() {
    const email = form.getValues("email");
    if (!email || !z.string().email().safeParse(email).success) return;

    setIsCheckingStatus(true);
    try {
      const result = await trpc.user.getAccountStatus.query({ email });
      setAccountStatus(result.status as any);

      if (result.status === "needs_password") {
        toast({
          title: "Account found",
          description: "Please claim your account to set your password.",
        });
      } else if (result.status === "absent") {
        toast({
          title: "Access Restricted",
          description:
            "Only invited users can access this platform. Please contact your administrator.",
          variant: "destructive",
        });
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsCheckingStatus(false);
    }
  }

  async function onSubmit(values: z.infer<typeof formSchema>) {
    // Re-check status if not yet checked
    if (!accountStatus) {
      await handleEmailBlur();
    }

    if (accountStatus === "absent") {
      toast({
        title: "Access Restricted",
        description:
          "Your email is not registered. Only invited users can join.",
        variant: "destructive",
      });
      return;
    }

    setIsLoading(true);

    try {
      if (accountStatus === "needs_password") {
        // Send a magic link to set up the account
        const { error } = await authClient.signIn.magicLink({
          email: values.email,
          callbackURL: "/account?setup=true",
        });

        if (error) {
          toast({
            title: "Error",
            description: error.message || "Failed to send setup link",
            variant: "destructive",
          });
        } else {
          toast({
            title: "Link Sent",
            description: "Check your email for a link to set your password.",
          });
        }
        return;
      }

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
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <div className="relative">
                  <Input
                    placeholder="Email address"
                    {...field}
                    autoCapitalize="none"
                    autoComplete="email"
                    autoCorrect="off"
                    spellCheck="false"
                    disabled={isLoading}
                    onBlur={(e) => {
                      field.onBlur();
                      handleEmailBlur();
                    }}
                  />
                  {isCheckingStatus && (
                    <div className="absolute right-3 top-3">
                      <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
                    </div>
                  )}
                </div>
                <FormMessage />
              </FormItem>
            )}
          />

          {accountStatus !== "needs_password" && (
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
                      autoComplete="current-password"
                      disabled={isLoading}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}

          <SubmitButton
            type="submit"
            className="w-full bg-primary text-secondary font-medium h-[40px]"
            isSubmitting={isLoading}
            disabled={accountStatus === "absent"}
          >
            {accountStatus === "needs_password" ? "Claim Account" : "Sign In"}
          </SubmitButton>
        </form>
      </Form>
    </div>
  );
}

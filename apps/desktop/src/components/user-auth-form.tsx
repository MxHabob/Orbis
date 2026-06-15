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
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { z } from "zod";

const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8, "Password must be at least 8 characters"),
});

type Props = {
  className?: string;
  mode?: "sign-in" | "sign-up";
};

export function UserAuthForm({ className, mode = "sign-in" }: Props) {
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsLoading(true);

    try {
      const { error } = await authClient.signIn.email({
        email: values.email,
        password: values.password,
      });

      if (error) {
        toast({
          title: "Error",
          description: error.message || "Invalid email or password",
          variant: "destructive",
        });
      } else {
        navigate("/", { replace: true });
      }
    } catch {
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
                <Input
                  placeholder="Email address"
                  {...field}
                  autoCapitalize="none"
                  autoComplete="email"
                  autoCorrect="off"
                  spellCheck="false"
                  disabled={isLoading}
                />
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
                    autoComplete="current-password"
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
            Sign In
          </SubmitButton>
        </form>
      </Form>
    </div>
  );
}

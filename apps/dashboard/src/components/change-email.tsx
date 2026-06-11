"use client";

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
import { z } from "zod/v3";
import { useUserMutation, useUserQuery } from "@/hooks/use-user";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";

const formSchema = z.object({
  email: z.string().email(),
});

export function ChangeEmail() {
  const { data: user } = useUserQuery();
  const updateUserMutation = useUserMutation();
  const t = useI18n();

  const form = useZodForm(formSchema, {
    defaultValues: {
      email: user?.email ?? undefined,
    },
  });

  const onSubmit = form.handleSubmit((data) => {
    updateUserMutation.mutate({
      email: data.email,
    });
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <Card>
          <CardHeader>
            <CardTitle>{t("account.email_title")}</CardTitle>
            <CardDescription>{t("account.email_description")}</CardDescription>
          </CardHeader>

          <CardContent>
            <FormField
              control={form.control}
              name="email"
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
                      type="email"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </CardContent>

          <CardFooter className="flex justify-between">
            <div>{t("account.email_primary_notice")}</div>
            <SubmitButton
              type="submit"
              isSubmitting={updateUserMutation.isPending}
            >
              {t("common.save")}
            </SubmitButton>
          </CardFooter>
        </Card>
      </form>
    </Form>
  );
}

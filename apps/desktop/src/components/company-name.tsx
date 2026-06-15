import { authClient } from "@/lib/auth-client";
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
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";

const formSchema = z.object({
  name: z.string().min(2).max(32),
});

export function CompanyName() {
  const { data, isPending } = authClient.useSession();
  const t = useI18n();

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
            <CardTitle>{t("settings.orbis_name_title")}</CardTitle>
            <CardDescription>
              {t("settings.orbis_name_description")}
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
            <div>{t("account.characters_limit", { count: 32 })}</div>
            <SubmitButton isSubmitting={isPending} disabled={isPending}>
              {t("common.save")}
            </SubmitButton>
          </CardFooter>
        </Card>
      </form>
    </Form>
  );
}

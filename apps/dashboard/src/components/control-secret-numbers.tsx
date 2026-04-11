"use client";

import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@midday/ui/card";
import { Button } from "@midday/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { useState } from "react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@midday/ui/alert-dialog";
import { Loader2 } from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import { useZodForm } from "@/hooks/use-zod-form";
import { z } from "zod/v3";

const schema = z.object({
  prefix: z.string().min(1),
});

export function ControlSecretNumbers() {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const [showConfirm, setShowConfirm] = useState(false);

  const form = useZodForm(schema, {
    defaultValues: {
      prefix: "",
    },
  });

  const mutation = useMutation(
    trpc.students.regenerateSecretNumbers.mutationOptions({
      onSuccess: () => {
        toast({ title: t("control.success_secret") });
        setShowConfirm(false);
      },
      onError: (err: any) => {
        toast({
          title: t("common.error"),
          description: err.message,
          variant: "destructive",
        });
      },
    }),
  );

  const onSubmit = form.handleSubmit(() => {
    setShowConfirm(true);
  });

  return (
    <>
      <Form {...form}>
        <form onSubmit={onSubmit}>
          <Card>
            <CardHeader>
              <CardTitle>{t("control.regenerate_secret")}</CardTitle>
              <CardDescription>
                {t("control.confirm_secret_desc")}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <FormField
                control={form.control}
                name="prefix"
                render={({ field }) => (
                  <FormItem>
                    <FormControl>
                      <Input
                        {...field}
                        placeholder={t("control.prefix_placeholder")}
                        className="max-w-[300px]"
                        autoComplete="off"
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </CardContent>
            <CardFooter className="flex justify-between">
              <div />
              <Button type="submit">
                {t("control.regenerate_secret")}
              </Button>
            </CardFooter>
          </Card>
        </form>
      </Form>

      <AlertDialog open={showConfirm} onOpenChange={setShowConfirm}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("control.confirm_title")}</AlertDialogTitle>
            <AlertDialogDescription>
              {t("control.confirm_secret_desc")}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={mutation.isPending}>
              {t("common.cancel")}
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={() => mutation.mutate(form.getValues())}
              disabled={mutation.isPending}
              className="bg-destructive hover:bg-destructive text-white"
            >
              {mutation.isPending && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              {t("common.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}

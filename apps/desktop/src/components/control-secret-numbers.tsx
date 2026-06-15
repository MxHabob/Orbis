"use client";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@orbis/ui/alert-dialog";
import { Button } from "@orbis/ui/button";
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
  FormLabel,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQuery } from "@tanstack/react-query";
import { Hash, HashIcon, Loader2 } from "lucide-react";
import { useState } from "react";
import { z } from "zod/v3";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/ipc/query-adapter";

const schema = z.object({
  classId: z.coerce.number().optional(),
  startFrom: z.coerce.number().int().min(0).optional(),
  pattern: z.string().optional(),
});

export function ControlSecretNumbers() {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const [showConfirm, setShowConfirm] = useState(false);

  const classesQuery = useQuery(
    trpc.classes.getAll.queryOptions({
      limit: 1000,
    }),
  );

  const form = useZodForm(schema, {
    defaultValues: {
      startFrom: 1,
      pattern: "",
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
              <CardTitle className="flex items-center gap-2">
                <Hash className="h-5 w-5" />
                {t("control.regenerate_secret")}
              </CardTitle>
              <CardDescription>
                {t("control.confirm_secret_desc")}
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <FormField
                  control={form.control}
                  name="classId"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("students.class")}</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value?.toString()}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue
                              placeholder={
                                t("control.class_placeholder") || "All Classes"
                              }
                            />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectItem value="0" key="all-classes">
                            {t("common.no_data") || "All Classes"}
                          </SelectItem>
                          {classesQuery.data?.data?.map((c) => (
                            <SelectItem key={c.id} value={c.id.toString()}>
                              {c.name}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="startFrom"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("control.start_number_label")}</FormLabel>
                      <FormControl>
                        <Input
                          {...field}
                          type="number"
                          placeholder="e.g. 721"
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              <FormField
                control={form.control}
                name="pattern"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>
                      {t("control.prefix_label")} (
                      {t("common.optional") || "Optional"})
                    </FormLabel>
                    <FormControl>
                      <Input
                        {...field}
                        placeholder={t("control.prefix_placeholder")}
                        autoComplete="off"
                      />
                    </FormControl>
                    <CardDescription className="text-xs mt-1">
                      {t("control.prefix_placeholder_desc")}
                    </CardDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </CardContent>
            <CardFooter className="flex justify-end">
              <Button type="submit" disabled={mutation.isPending}>
                {mutation.isPending && (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                )}
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
              onClick={() => {
                const values = form.getValues();
                mutation.mutate({
                  ...values,
                  classId: values.classId === 0 ? undefined : values.classId,
                  startFrom: values.startFrom || 1,
                });
              }}
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

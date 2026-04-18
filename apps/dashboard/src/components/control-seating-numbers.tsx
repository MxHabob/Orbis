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
  FormLabel,
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { Loader2, Users } from "lucide-react";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useZodForm } from "@/hooks/use-zod-form";
import { z } from "zod/v3";

const schema = z.object({
  classId: z.coerce.number().optional(),
  startFrom: z.coerce.number().int().min(0),
});

export function ControlSeatingNumbers() {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const [showConfirm, setShowConfirm] = useState(false);

  const classesQuery = useQuery(trpc.classes.getAll.queryOptions({
    limit: 1000,
  }));

  const form = useZodForm(schema, {
    defaultValues: {
      startFrom: 1,
    },
  });

  const mutation = useMutation(
    trpc.students.regenerateSeatingNumbers.mutationOptions({
      onSuccess: () => {
        toast({ title: t("control.success_seating") });
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
                <Users className="h-5 w-5" />
                {t("control.regenerate_seating")}
              </CardTitle>
              <CardDescription>
                {t("control.confirm_seating_desc")}
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
                            <SelectValue placeholder={t("control.class_placeholder") || "All Classes"} />
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
                          placeholder={t("control.start_number_placeholder")}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
            </CardContent>
            <CardFooter className="flex justify-end">
              <Button type="submit" disabled={mutation.isPending}>
                {mutation.isPending && (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                )}
                {t("control.regenerate_seating")}
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
              {t("control.confirm_seating_desc")}
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


"use client";

import { useUserQuery } from "@/hooks/use-user";
import { useTRPC } from "@/trpc/client";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@midday/ui/alert-dialog";
import { Button } from "@midday/ui/button";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@midday/ui/card";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Loader2 } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useI18n } from "@/locales/client";

export function DeleteTeam() {
  const [value, setValue] = useState("");
  const trpc = useTRPC();
  const { data: user } = useUserQuery();
  const router = useRouter();
  const t = useI18n();


  return (
    <Card className="border-destructive">
      <CardHeader>
        <CardTitle>{t("settings.delete_team_title")}</CardTitle>
        <CardDescription>
          {t("settings.delete_team_description")}
        </CardDescription>
      </CardHeader>
      <CardFooter className="flex justify-between">
        <div />

        <AlertDialog>
          <AlertDialogTrigger asChild>
            <Button
              variant="destructive"
              className="hover:bg-destructive text-muted"
            >
              {t("common.delete")}
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>{t("settings.delete_team_confirm_title")}</AlertDialogTitle>
              <AlertDialogDescription asChild>
                <div>
                  <p>
                    {t("settings.delete_team_confirm_description")}
                  </p>
                </div>
              </AlertDialogDescription>
            </AlertDialogHeader>

            <div className="flex flex-col gap-2 mt-2">
              <Label htmlFor="confirm-delete">
                <span dangerouslySetInnerHTML={{ __html: t("settings.delete_team_input_label", { delete: `<span class="font-medium">DELETE</span>` }) }} />
              </Label>
              <Input
                id="confirm-delete"
                value={value}
                onChange={(e) => setValue(e.target.value)}
              />
            </div>

            <AlertDialogFooter>
              <AlertDialogCancel>{t("common.cancel")}</AlertDialogCancel>
              <AlertDialogAction
                disabled={value !== "DELETE"}
              >
                {value !== "DELETE" ? (
                  <Loader2 className="h-4 w-4 animate-spin" />
                ) : (
                  t("common.confirm")
                )}
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </CardFooter>
    </Card>
  );
}

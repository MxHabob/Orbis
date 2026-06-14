"use client";

import { Button } from "@orbis/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { Textarea } from "@orbis/ui/textarea";
import { Loader2 } from "lucide-react";
import { useAction } from "next-safe-action/hooks";
import { useState } from "react";
import { sendFeebackAction } from "@/actions/send-feedback-action";
import { useI18n } from "@/locales/client";

export function SupportForm() {
  const t = useI18n();
  const [value, setValue] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const action = useAction(sendFeebackAction, {
    onSuccess: () => {
      setValue("");
      setSubmitted(true);

      setTimeout(() => {
        setSubmitted(false);
      }, 5000);
    },
  });

  return (
    <div className="space-y-12">
      <Card>
        <CardHeader>
          <CardTitle>{t("settings.support")}</CardTitle>
          <CardDescription>{t("settings.support_description")}</CardDescription>
        </CardHeader>

        <CardContent>
          <form className="space-y-4">
            <Textarea
              name="feedback"
              value={value}
              required
              placeholder={t("settings.support_message_placeholder")}
              className="resize-none h-[150px]"
              onChange={(evt) => setValue(evt.target.value)}
            />

            {submitted && (
              <p className="font-medium text-sm text-green-600 mt-2">
                {t("settings.support_success")}
              </p>
            )}
          </form>
        </CardContent>

        <CardFooter className="flex justify-between">
          <div />
          <Button
            type="button"
            onClick={() => action.execute({ feedback: value })}
            disabled={value.length === 0 || action.status === "executing"}
          >
            {action.status === "executing" ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              t("settings.send")
            )}
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}

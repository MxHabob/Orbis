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
import { useSafeAction } from "@/hooks/use-safe-action";
import { useState } from "react";
import { useI18n } from "@/locales/client";

async function submitFeedback(input: { feedback: string }) {
  console.info("[desktop] feedback submitted", input.feedback.slice(0, 120));
  return { ok: true as const };
}

export function SupportForm() {
  const t = useI18n();
  const [value, setValue] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const action = useSafeAction(submitFeedback, {
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
            disabled={value.length === 0 || action.isExecuting}
          >
            {action.isExecuting ? (
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

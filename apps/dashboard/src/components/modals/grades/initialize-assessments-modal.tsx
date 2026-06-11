"use client";

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { parseAsInteger, parseAsString, useQueryStates } from "nuqs";
import { AssessmentsForm } from "@/components/forms/assessments-form";
import { useI18n } from "@/locales/client";

export function InitializeAssessmentsModal() {
  const t = useI18n();
  const [params, setParams] = useQueryStates({
    step: parseAsString,
    initialize_classId: parseAsInteger,
    initialize_subjectId: parseAsInteger,
    initialize_termId: parseAsInteger,
    initialize_month: parseAsInteger,
  });

  const isOpen = params.step === "initialize_assessments";
  const {
    initialize_classId: classId,
    initialize_subjectId: subjectId,
    initialize_termId: termId,
    initialize_month: month,
  } = params;

  const onclose = () => {
    setParams({
      step: null,
      initialize_classId: null,
      initialize_subjectId: null,
      initialize_termId: null,
      initialize_month: null,
    });
  };

  return (
    <Dialog open={isOpen} onOpenChange={onclose}>
      <DialogContent className="max-w-[780px]">
        <div className="p-2">
          <DialogHeader className="mb-4">
            <DialogTitle className="text-xl font-semibold">
              {t("grades.initialize_assessments")}
            </DialogTitle>
            <DialogDescription>
              {t("grades.initialize_assessments_description")}
            </DialogDescription>
          </DialogHeader>

          {classId && subjectId && termId && month && (
            <AssessmentsForm
              classId={classId}
              subjectId={subjectId}
              termId={termId}
              month={month}
              onSuccessAction={onclose}
            />
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}

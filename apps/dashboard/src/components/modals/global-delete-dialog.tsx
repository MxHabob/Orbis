"use client";

import { useI18n } from "@/locales/client";
import { useStudentParams } from "@/hooks/use-student-params";
import { useClassParams } from "@/hooks/use-class-params";
import { useSubjectParams } from "@/hooks/use-subject-params";
import { useStaffParams } from "@/hooks/use-staff-params";
import { useAcademicParams } from "@/hooks/use-academic-params";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { DeleteDialog } from "./delete-dialog";
import { invalidateStudentDomain, invalidateClassDomain, invalidateSubjectDomain } from "@/utils/query-invalidation";
import { useToast } from "@midday/ui/use-toast";

export function GlobalDeleteDialog() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const t = useI18n();
  const { toast } = useToast();

  const { deleteStudentId, setParams: setStudentParams, studentId } = useStudentParams();
  const { deleteClassId, setParams: setClassParams } = useClassParams();
  const { deleteSubjectId, setParams: setSubjectParams } = useSubjectParams();
  const { deleteStaffId, setParams: setStaffParams } = useStaffParams();
  const { deleteYearId, deleteTermId, setParams: setAcademicParams } = useAcademicParams();

  // Mutations
  const deleteStudentMutation = useMutation(
    trpc.students.delete.mutationOptions({
      onSuccess: () => {
        void invalidateStudentDomain(queryClient, trpc);
        setStudentParams({ studentId: null, deleteStudentId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  const deleteClassMutation = useMutation(
    trpc.classes.delete.mutationOptions({
      onSuccess: () => {
        void invalidateClassDomain(queryClient, trpc, Number(deleteClassId));
        setClassParams({ classId: null, deleteClassId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  const deleteSubjectMutation = useMutation(
    trpc.subjects.delete.mutationOptions({
      onSuccess: () => {
        void invalidateSubjectDomain(queryClient, trpc, Number(deleteSubjectId));
        setSubjectParams({ subjectId: null, deleteSubjectId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  const deleteStaffMutation = useMutation(
    trpc.user.delete.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.user.getStaff.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.user.getParents.queryKey(),
        });
        setStaffParams({ deleteStaffId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  const deleteYearMutation = useMutation(
    trpc.academic.deleteYear.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
        setAcademicParams({ deleteYearId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  const deleteTermMutation = useMutation(
    trpc.academic.deleteTerm.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
        setAcademicParams({ deleteTermId: null });
        toast({ title: t("common.success"), variant: "success" });
      },
    })
  );

  // Handlers
  const handleConfirm = () => {
    if (deleteStudentId) {
      deleteStudentMutation.mutate({ userId: deleteStudentId });
    } else if (deleteClassId) {
      deleteClassMutation.mutate({ id: Number(deleteClassId) });
    } else if (deleteSubjectId) {
      deleteSubjectMutation.mutate({ id: Number(deleteSubjectId) });
    } else if (deleteStaffId) {
      deleteStaffMutation.mutate({ id: deleteStaffId });
    } else if (deleteYearId) {
      deleteYearMutation.mutate({ id: Number(deleteYearId) });
    } else if (deleteTermId) {
      deleteTermMutation.mutate({ id: Number(deleteTermId) });
    }
  };

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      if (deleteStudentId) setStudentParams({ deleteStudentId: null });
      if (deleteClassId) setClassParams({ deleteClassId: null });
      if (deleteSubjectId) setSubjectParams({ deleteSubjectId: null });
      if (deleteStaffId) setStaffParams({ deleteStaffId: null });
      if (deleteYearId) setAcademicParams({ deleteYearId: null });
      if (deleteTermId) setAcademicParams({ deleteTermId: null });
    }
  };

  // Determine current active dialog configuration
  const config = {
    open: Boolean(
      deleteStudentId || 
      deleteClassId || 
      deleteSubjectId || 
      deleteStaffId ||
      deleteYearId ||
      deleteTermId
    ),
    title: "",
    description: "",
    isPending: deleteStudentMutation.isPending || 
               deleteClassMutation.isPending || 
               deleteSubjectMutation.isPending || 
               deleteStaffMutation.isPending ||
               deleteYearMutation.isPending ||
               deleteTermMutation.isPending,
  };

  if (deleteStudentId) {
    config.title = t("students.delete_student");
    config.description = t("students.delete_description");
  } else if (deleteClassId) {
    config.title = t("education.delete_title");
    config.description = t("education.delete_description");
  } else if (deleteSubjectId) {
    config.title = t("subjects.delete_title");
    config.description = t("subjects.delete_description");
  } else if (deleteStaffId) {
    config.title = t("staff.delete_title");
    config.description = t("staff.delete_description");
  } else if (deleteYearId) {
    config.title = t("education.delete_year_title");
    config.description = t("education.delete_year_description");
  } else if (deleteTermId) {
    config.title = t("education.delete_term_title");
    config.description = t("education.delete_term_description");
  }

  return (
    <DeleteDialog
      open={config.open}
      title={config.title}
      description={config.description}
      isPending={config.isPending}
      onConfirm={handleConfirm}
      onOpenChange={handleOpenChange}
    />
  );
}

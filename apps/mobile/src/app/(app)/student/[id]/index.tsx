/**
 * @module @orbis/mobile/app/(app)/student/[id]/index
 *
 * Comprehensive Student Profile & Academic Hub.
 * Features hero identity, academic metric widgets, two-column action launcher,
 * and detailed biographical records.
 */

import React from "react";
import { View, Text } from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar } from "@/components/ui/avatar";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import {
  useLinkedStudentId,
  useStudentQuery,
} from "@/hooks/use-api-queries";
import {
  AcademicCap,
  Calendar,
  ChartBar,
  ClipboardDocumentCheck,
  CreditCard,
  DocumentText,
  User,
  Phone,
  Envelope,
  ChevronLeft,
  ChevronRight,
} from "@orbis/icons/native";

const studentValueKeys: Partial<Record<string, TranslationKey>> = {
  active: "student_status_active",
  inactive: "student_status_inactive",
  on_leave: "student_status_on_leave",
  suspended: "student_status_suspended",
  withdrawn: "student_status_withdrawn",
  transferred_out: "student_status_transferred_out",
  graduated: "student_status_graduated",
  expelled: "student_status_expelled",
  male: "student_gender_male",
  female: "student_gender_female",
};

export default function StudentIndexScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic, formatDate } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityMissing,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);

  const studentQuery = useStudentQuery(studentId);
  const student = studentQuery.data;

  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

  const fullName = student
    ? isArabic
      ? student.name_ar || student.full_name
      : student.display_name || student.full_name
    : t("student_profile");

  const studentValue = (value: string) => {
    const key = studentValueKeys[value.toLowerCase()];
    return key ? t(key) : value;
  };

  return (
    <StudentRecordLayout
      title={fullName}
      showBack={true}
      isPending={isResolving || (isLinked && studentQuery.isPending)}
      isError={!isResolving && (!isLinked || studentQuery.isError)}
      canRetry={isLinked}
      errorTitle={identityMissing ? t("student_profile_missing") : undefined}
      errorDescription={
        isLinked || isResolving || identityMissing
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? studentQuery.refetch() : retryIdentity())
      }
    >
      {student ? (
        <>
          {/* 1. Hero Identity Card */}
          <Card
            variant="default"
            padding="four"
          >
            <View className="flex-row items-center gap-four">
              <Avatar
                fallback={(student.full_name || "S").charAt(0)}
                size="lg"
                className="h-16 w-16 border-2 border-primary/20 bg-primary/10"
              />
              <View className="flex-1 gap-half">
                <Text className="text-lg font-extrabold text-foreground">
                  {fullName}
                </Text>
                <View className="flex-row items-center gap-two">
                  <Badge variant="secondary" className="px-two py-half">
                    <Text className="text-xs font-bold text-primary">
                      {student.student_number || `ID: #${student.id.slice(0, 6)}`}
                    </Text>
                  </Badge>
                  <Badge variant="outline" className="px-two py-half">
                    <Text className="text-[10px] font-semibold text-muted-foreground">
                      {studentValue(student.status)}
                    </Text>
                  </Badge>
                </View>
              </View>
            </View>
          </Card>

          {/* 2. Academic Modules Action Grid (2-column layout) */}
          <View className="gap-three">
            <Text className="text-base font-bold text-foreground">
              {t("home_services") || "Academic Portals"}
            </Text>

            <View className="flex-row gap-three">
              <ModuleGridCard
                icon={<ChartBar size={22} color={theme.primary} />}
                title={t("student_grades")}
                subtitle={t("domain_grades")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/grades",
                    params: { id: student.id },
                  })
                }
              />
              <ModuleGridCard
                icon={<ClipboardDocumentCheck size={22} color={theme.primary} />}
                title={t("student_attendance")}
                subtitle={t("domain_attendance")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/attendance",
                    params: { id: student.id },
                  })
                }
              />
            </View>

            <View className="flex-row gap-three">
              <ModuleGridCard
                icon={<Calendar size={22} color={theme.primary} />}
                title={t("student_schedule")}
                subtitle={t("domain_schedule")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/schedule",
                    params: { id: student.id },
                  })
                }
              />
              <ModuleGridCard
                icon={<AcademicCap size={22} color={theme.primary} />}
                title={t("student_exams")}
                subtitle={t("student_exams_description")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/exams",
                    params: { id: student.id },
                  })
                }
              />
            </View>

            <View className="flex-row gap-three">
              <ModuleGridCard
                icon={<CreditCard size={22} color={theme.primary} />}
                title={t("student_fees")}
                subtitle={t("student_fees_description")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/fees",
                    params: { id: student.id },
                  })
                }
              />
            </View>
          </View>

          {/* 3. Biographical & Contact Information */}
          <View className="gap-three pt-two">
            <Text className="text-base font-bold text-foreground">
              {t("profile_title") || "Student Information"}
            </Text>

            <Card variant="default" padding="four" className="gap-three">
              {student.date_of_birth ? (
                <DetailRow
                  label={t("student_birth_date")}
                  value={formatDate(student.date_of_birth)}
                />
              ) : null}
              {student.gender ? (
                <DetailRow
                  label={t("student_gender")}
                  value={studentValue(student.gender)}
                />
              ) : null}
              {student.email ? (
                <DetailRow
                  label={t("student_email")}
                  value={student.email}
                />
              ) : null}
              {student.phone ? (
                <DetailRow
                  label={t("student_phone")}
                  value={student.phone}
                />
              ) : null}
            </Card>
          </View>
        </>
      ) : null}
    </StudentRecordLayout>
  );
}

function ModuleGridCard({
  icon,
  title,
  subtitle,
  onPress,
}: {
  icon: React.ReactNode;
  title: string;
  subtitle: string;
  onPress: () => void;
}) {
  return (
    <Card
      variant="interactive"
      padding="four"
      accessibilityRole="button"
      accessibilityLabel={`${title}. ${subtitle}`}
      onPress={onPress}
      className="min-h-28 flex-1 gap-three"
    >
      <View className="h-10 w-10 items-center justify-center rounded-md bg-primary/10">
        {icon}
      </View>
      <View className="gap-half">
        <Text className="text-sm font-bold text-foreground">{title}</Text>
        <Text className="text-xs text-muted-foreground" numberOfLines={1}>
          {subtitle}
        </Text>
      </View>
    </Card>
  );
}

function DetailRow({ label, value }: { label: string; value: string }) {
  return (
    <View className="flex-row items-center justify-between border-b border-border/40 pb-two">
      <Text className="text-xs font-medium text-muted-foreground">{label}</Text>
      <Text className="text-xs font-bold text-foreground">{value}</Text>
    </View>
  );
}

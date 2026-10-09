import { FeatureCard } from "@/components/feature-card";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import {
  useLinkedStudentId,
  useStudentQuery,
} from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useLocalSearchParams, useRouter } from "expo-router";
import { Text, View } from "react-native";

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
  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityError,
    retryIdentity,
  } =
    useLinkedStudentId(requestedStudentId);
  const studentQuery = useStudentQuery(studentId);
  const { t, isArabic, formatDate } = useI18n();
  const router = useRouter();
  const student = studentQuery.data;
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
      isPending={isResolving || (isLinked && studentQuery.isPending)}
      isError={!isResolving && (!isLinked || studentQuery.isError)}
      canRetry
      errorDescription={
        isLinked || isResolving || identityError
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? studentQuery.refetch() : retryIdentity())
      }
    >
      {student ? (
        <>
          <Card variant="outline" padding="four">
            <View className="gap-three">
              <Text className="text-lg font-semibold text-foreground">
                {t("student_profile")}
              </Text>
              <ProfileField label={t("student_number")} value={student.student_number} />
              <ProfileField
                label={t("student_status")}
                value={studentValue(student.status)}
              />
              {student.date_of_birth ? (
                <ProfileField
                  label={t("student_birth_date")}
                  value={formatDate(student.date_of_birth)}
                />
              ) : null}
              <ProfileField
                label={t("student_gender")}
                value={studentValue(student.gender)}
              />
              {student.email ? (
                <ProfileField label={t("student_email")} value={student.email} />
              ) : null}
              {student.phone ? (
                <ProfileField label={t("student_phone")} value={student.phone} />
              ) : null}
            </View>
          </Card>
          <View className="gap-three">
            <FeatureCard
              title={t("student_grades")}
              description={t("domain_grades")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/student/[id]/grades",
                  params: { id: student.id },
                })
              }
            />
            <FeatureCard
              title={t("student_attendance")}
              description={t("domain_attendance")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/student/[id]/attendance",
                  params: { id: student.id },
                })
              }
            />
            <FeatureCard
              title={t("student_schedule")}
              description={t("domain_schedule")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/student/[id]/schedule",
                  params: { id: student.id },
                })
              }
            />
            <FeatureCard
              title={t("student_exams")}
              description={t("student_exams_description")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/student/[id]/exams",
                  params: { id: student.id },
                })
              }
            />
            <FeatureCard
              title={t("student_fees")}
              description={t("student_fees_description")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/student/[id]/fees",
                  params: { id: student.id },
                })
              }
            />
          </View>
        </>
      ) : null}
    </StudentRecordLayout>
  );
}

function ProfileField({ label, value }: { label: string; value: string }) {
  return (
    <View className="flex-row items-start justify-between gap-three border-b border-border pb-two">
      <Text className="flex-1 text-sm text-muted-foreground">{label}</Text>
      <Text className="flex-1 text-end text-sm font-medium text-foreground">
        {value}
      </Text>
    </View>
  );
}

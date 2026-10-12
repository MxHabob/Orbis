import { FeatureCard } from "@/components/feature-card";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { useTeacherClassDataQuery } from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import { useLocalSearchParams, useRouter } from "expo-router";
import { Text, View } from "react-native";

export default function ClassIndexScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const classQuery = useTeacherClassDataQuery(classId);
  const { t, isArabic } = useI18n();
  const router = useRouter();
  const classItem = classQuery.data?.classItem;
  const className = classItem
    ? isArabic
      ? classItem.name_ar || classItem.name
      : classItem.name
    : t("teacher_class_roster");

  return (
    <StudentRecordLayout
      title={className}
      isPending={Boolean(classId) && classQuery.isPending}
      isError={!classId || classQuery.isError}
      onRetry={() => void classQuery.refetch()}
    >
      {classQuery.data ? (
        <>
          <Card variant="outline" padding="four">
            <Text className="text-sm leading-5 text-muted-foreground">
              {t("teacher_students_count", {
                count: classQuery.data.roster.length,
              })}
            </Text>
          </Card>
          <View className="gap-three">
            <FeatureCard
              title={t("teacher_mark_attendance")}
              description={t("student_attendance")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/class/[id]/attendance",
                  params: { id: classQuery.data.classItem.id },
                })
              }
            />
            <FeatureCard
              title={t("teacher_record_grades")}
              description={t("student_grades")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/class/[id]/grades",
                  params: { id: classQuery.data.classItem.id },
                })
              }
            />
            <FeatureCard
              title={t("teacher_class_schedule")}
              description={t("student_schedule")}
              onPress={() =>
                router.push({
                  pathname: "/(app)/class/[id]/schedule",
                  params: { id: classQuery.data.classItem.id },
                })
              }
            />
          </View>
          <Text className="text-lg font-bold text-foreground">
            {t("teacher_class_roster")}
          </Text>
          {classQuery.data.roster.length === 0 ? (
            <EmptyState title={t("teacher_no_students")} />
          ) : (
            <View className="gap-two">
              {classQuery.data.roster.map(({ student, enrollment }) => (
                <Card key={student.id} variant="outline" padding="three">
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="flex-1 text-sm font-semibold text-foreground">
                      {isArabic
                        ? student.name_ar || student.full_name
                        : student.display_name || student.full_name}
                    </Text>
                    <Text className="text-xs text-muted-foreground">
                      {enrollment.student_number}
                    </Text>
                  </View>
                </Card>
              ))}
            </View>
          )}
        </>
      ) : null}
    </StudentRecordLayout>
  );
}

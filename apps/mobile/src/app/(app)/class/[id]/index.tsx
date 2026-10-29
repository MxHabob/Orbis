/**
 * @module @orbis/mobile/app/(app)/class/[id]/index
 *
 * Class Hub & Roster Dashboard for Teachers in Orbis Mobile.
 * Features class metadata metrics, quick action launcher for roll call/grades,
 * and roster directory.
 */

import { View, Text } from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar } from "@/components/ui/avatar";
import { EmptyState } from "@/components/ui/empty-state";
import { ListItem } from "@/components/ui/list-item";
import { useTeacherClassDataQuery } from "@/hooks/use-api-queries";
import type { TeacherRosterEntry } from "@/hooks/use-api-queries";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  Users,
  ClipboardDocumentCheck,
  ChartBar,
  Calendar,
} from "@orbis/icons/native";

export default function ClassIndexScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const classQuery = useTeacherClassDataQuery(classId);
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();

  const classItem = classQuery.data?.classItem;
  const className = classItem
    ? isArabic
      ? classItem.name_ar || classItem.name
      : classItem.name
    : t("teacher_class_roster");

  const roster = classQuery.data?.roster ?? [];

  return (
    <StudentRecordLayout
      title={className}
      showBack={true}
      isPending={classQuery.isInScope && classQuery.isPending}
      isError={!classQuery.isInScope || classQuery.isError}
      canRetry={classQuery.isInScope}
      errorDescription={
        classQuery.isInScope ? undefined : t("class_access_denied")
      }
      onRetry={() => void classQuery.refetch()}
    >
      {classQuery.data ? (
        <>
          {/* 1. Class Metric Banner */}
          <Card variant="default" padding="four">
            <View className="flex-row items-center justify-between">
              <View className="gap-half">
                <Text className="text-xl font-extrabold text-foreground">
                  {className}
                </Text>
                <Text className="text-xs font-medium text-muted-foreground">
                  {t("home_class_level")} {classItem?.level}
                </Text>
              </View>

              <Badge
                variant="secondary"
                className="flex-row items-center gap-one px-three py-one"
              >
                <Users size={14} color={theme.primary} />
                <Text className="text-xs font-bold text-primary">
                  {formatNumber(roster.length)}{" "}
                  {t("teacher_students_count_short") || "Students"}
                </Text>
              </Badge>
            </View>
          </Card>

          {/* 2. Teacher Quick Actions (3 buttons) */}
          <View className="gap-two">
            <Text className="text-base font-bold text-foreground">
              {t("home_services") || "Class Actions"}
            </Text>

            <View className="flex-row gap-three">
              <Card
                variant="interactive"
                padding="four"
                accessibilityRole="button"
                accessibilityLabel={t("teacher_mark_attendance")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/attendance",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="min-h-24 flex-1 items-center justify-center gap-two"
              >
                <ClipboardDocumentCheck size={24} color={theme.primary} />
                <Text className="text-center text-xs font-bold text-primary">
                  {t("teacher_mark_attendance")}
                </Text>
              </Card>

              <Card
                variant="interactive"
                padding="four"
                accessibilityRole="button"
                accessibilityLabel={t("teacher_record_grades")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/grades",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="min-h-24 flex-1 items-center justify-center gap-two"
              >
                <ChartBar size={24} color={theme.foreground} />
                <Text className="text-center text-xs font-bold text-foreground">
                  {t("teacher_record_grades")}
                </Text>
              </Card>

              <Card
                variant="interactive"
                padding="four"
                accessibilityRole="button"
                accessibilityLabel={t("teacher_class_schedule")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/schedule",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="min-h-24 flex-1 items-center justify-center gap-two"
              >
                <Calendar size={24} color={theme.foreground} />
                <Text className="text-center text-xs font-bold text-foreground">
                  {t("teacher_class_schedule")}
                </Text>
              </Card>
            </View>
          </View>

          {/* 3. Student Roster Directory */}
          <View className="gap-three pt-two">
            <View className="flex-row items-center justify-between">
              <Text className="text-base font-bold text-foreground">
                {t("teacher_class_roster")}
              </Text>
              <Text className="text-xs font-semibold text-muted-foreground">
                {formatNumber(roster.length)}{" "}
                {t("teacher_students_count_short") || "Enrolled"}
              </Text>
            </View>

            {roster.length === 0 ? (
              <EmptyState title={t("teacher_no_students")} />
            ) : (
              <Card variant="default" padding="one">
                {roster.map(
                  (
                    { student, enrollment }: TeacherRosterEntry,
                    index: number,
                  ) => {
                    const studentName = isArabic
                      ? student.name_ar || student.full_name
                      : student.display_name || student.full_name;

                    return (
                      <View key={student.id}>
                        {index > 0 ? (
                          <View className="mx-three h-px bg-border" />
                        ) : null}
                        <ListItem
                          title={studentName}
                          subtitle={
                            enrollment.student_number ||
                            `ID: #${student.id.slice(0, 6)}`
                          }
                          left={
                            <Avatar
                              fallback={(studentName || "S").charAt(0)}
                              size="sm"
                              className="bg-primary/10"
                            />
                          }
                          chevron
                          onPress={() =>
                            router.push({
                              pathname: "/(app)/student/[id]",
                              params: { id: student.id },
                            })
                          }
                        />
                      </View>
                    );
                  },
                )}
              </Card>
            )}
          </View>
        </>
      ) : null}
    </StudentRecordLayout>
  );
}

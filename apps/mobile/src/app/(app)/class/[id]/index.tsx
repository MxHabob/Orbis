/**
 * @module @orbis/mobile/app/(app)/class/[id]/index
 *
 * Class Hub & Roster Dashboard for Teachers in Orbis Mobile.
 * Features class metadata metrics, quick action launcher for roll call/grades,
 * and roster directory.
 */

import { View, Text, Pressable } from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar } from "@/components/ui/avatar";
import { EmptyState } from "@/components/ui/empty-state";
import { useTeacherClassDataQuery } from "@/hooks/use-api-queries";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  Users,
  ClipboardDocumentCheck,
  ChartBar,
  Calendar,
  ChevronLeft,
  ChevronRight,
  AcademicCap,
} from "@orbis/icons/native";

export default function ClassIndexScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const classQuery = useTeacherClassDataQuery(classId);
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();

  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

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
      isPending={Boolean(classId) && classQuery.isPending}
      isError={!classId || classQuery.isError}
      onRetry={() => void classQuery.refetch()}
    >
      {classQuery.data ? (
        <>
          {/* 1. Class Metric Banner */}
          <Card
            variant="default"
            padding="four"
            className="bg-card border border-border/80 shadow-sm"
          >
            <View className="flex-row items-center justify-between">
              <View className="gap-half">
                <Text className="text-xl font-extrabold text-foreground">
                  {className}
                </Text>
                <Text className="text-xs font-medium text-muted-foreground">
                  {t("home_class_level")} {classItem?.level}
                </Text>
              </View>

              <Badge variant="secondary" className="flex-row items-center gap-one px-three py-one">
                <Users size={14} color={theme.primary} />
                <Text className="text-xs font-bold text-primary">
                  {formatNumber(roster.length)} {t("teacher_students_count_short") || "Students"}
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
              <Pressable
                accessibilityRole="button"
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/attendance",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="flex-1 items-center gap-two rounded-2xl border border-primary/20 bg-primary/10 p-four shadow-xs active:opacity-70"
              >
                <ClipboardDocumentCheck size={24} color={theme.primary} />
                <Text className="text-center text-xs font-bold text-primary">
                  {t("teacher_mark_attendance")}
                </Text>
              </Pressable>

              <Pressable
                accessibilityRole="button"
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/grades",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="flex-1 items-center gap-two rounded-2xl border border-border/80 bg-card p-four shadow-xs active:opacity-70"
              >
                <ChartBar size={24} color={theme.foreground} />
                <Text className="text-center text-xs font-bold text-foreground">
                  {t("teacher_record_grades")}
                </Text>
              </Pressable>

              <Pressable
                accessibilityRole="button"
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]/schedule",
                    params: { id: classQuery.data.classItem.id },
                  })
                }
                className="flex-1 items-center gap-two rounded-2xl border border-border/80 bg-card p-four shadow-xs active:opacity-70"
              >
                <Calendar size={24} color={theme.foreground} />
                <Text className="text-center text-xs font-bold text-foreground">
                  {t("teacher_class_schedule")}
                </Text>
              </Pressable>
            </View>
          </View>

          {/* 3. Student Roster Directory */}
          <View className="gap-three pt-two">
            <View className="flex-row items-center justify-between">
              <Text className="text-base font-bold text-foreground">
                {t("teacher_class_roster")}
              </Text>
              <Text className="text-xs font-semibold text-muted-foreground">
                {formatNumber(roster.length)} {t("teacher_students_count_short") || "Enrolled"}
              </Text>
            </View>

            {roster.length === 0 ? (
              <EmptyState title={t("teacher_no_students")} />
            ) : (
              <View className="gap-two">
                {roster.map(({ student, enrollment }) => {
                  const studentName = isArabic
                    ? student.name_ar || student.full_name
                    : student.display_name || student.full_name;

                  return (
                    <Pressable
                      key={student.id}
                      accessibilityRole="button"
                      onPress={() =>
                        router.push({
                          pathname: "/(app)/student/[id]",
                          params: { id: student.id },
                        })
                      }
                      className="flex-row items-center justify-between rounded-xl border border-border/70 bg-card p-three shadow-2xs active:bg-secondary/40"
                    >
                      <View className="flex-row items-center gap-three">
                        <Avatar
                          fallback={(studentName || "S").charAt(0)}
                          size="sm"
                          className="bg-primary/10"
                        />
                        <View className="gap-half">
                          <Text className="text-sm font-bold text-foreground">
                            {studentName}
                          </Text>
                          <Text className="text-xs text-muted-foreground">
                            {enrollment.student_number || `ID: #${student.id.slice(0, 6)}`}
                          </Text>
                        </View>
                      </View>

                      <DirectionChevron size={16} color={theme.mutedForeground} />
                    </Pressable>
                  );
                })}
              </View>
            )}
          </View>
        </>
      ) : null}
    </StudentRecordLayout>
  );
}

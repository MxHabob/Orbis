import { BookOpen, ChatBubbles } from "@orbis/icons/native";
import {
  AssignmentChecklist,
  Assignments,
  Attendance,
  Grades,
  Schedule,
  StudentBoy,
} from "@orbis/icons/native/Illustrations";
import { useRouter } from "expo-router";
import { useMemo, useState } from "react";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { HEADER_CONTENT_OFFSET } from "@/components/header";
import { HomeHero } from "@/components/home/home-hero";
import {
  HomeSection,
  HomeTileGrid,
  HomeTileSkeletonGrid,
  QuickActionRow,
} from "@/components/home/home-sections";
import type { HomeTileData } from "@/components/home/types";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { EmptyState } from "@/components/ui/empty-state";
import { SegmentedControl } from "@/components/ui/segmented-control";
import { useClassesQuery, useMyStudentQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import type { ClassDto } from "@/lib/api/types";
import { useI18n } from "@/lib/i18n";

type RoleMode = "parent" | "student" | "teacher";

function getRoleMode(role: string): RoleMode | null {
  const normalizedRole = role.toLowerCase().replace(/[^a-z]/g, "");
  if (normalizedRole.includes("teacher")) return "teacher";
  if (normalizedRole.includes("student")) return "student";
  if (
    normalizedRole.includes("guardian") ||
    normalizedRole.includes("parent") ||
    normalizedRole.includes("family")
  ) {
    return "parent";
  }
  return null;
}

export default function HomeTabScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t, isArabic, l } = useI18n();
  const { user, effectiveContext } = useAuth();

  const availableRoles = useMemo(() => {
    const detected = new Set(
      [
        ...(user?.roles ?? []),
        effectiveContext?.effective_role ?? "",
        ...(effectiveContext?.roles ?? []),
      ]
        .map(getRoleMode)
        .filter((role): role is RoleMode => role !== null),
    );
    if (detected.size === 0) detected.add("parent");
    return (["parent", "student", "teacher"] as const).filter((role) =>
      detected.has(role),
    );
  }, [effectiveContext?.effective_role, effectiveContext?.roles, user?.roles]);
  const initialRole = availableRoles.includes("teacher")
    ? "teacher"
    : availableRoles.includes("student")
      ? "student"
      : "parent";
  const [selectedRole, setSelectedRole] = useState<RoleMode>(initialRole);
  const activeRole = availableRoles.includes(selectedRole)
    ? selectedRole
    : initialRole;
  const classesQuery = useClassesQuery(activeRole === "teacher");
  const studentQuery = useMyStudentQuery(activeRole === "student");
  const dependents = effectiveContext?.dependents ?? [];

  const displayName = isArabic
    ? user?.full_name_ar || user?.full_name
    : user?.full_name || user?.full_name_ar;
  const firstName = displayName?.trim().split(/\s+/)[0];
  const roleOptions = availableRoles.map((role) => ({
    value: role,
    label: t(
      role === "parent"
        ? "auth_role_parent"
        : role === "student"
          ? "auth_role_student"
          : "auth_role_teacher",
    ),
  }));
  const parentChildren: HomeTileData[] = dependents.map((dependent) => ({
    id: dependent.student_id,
    title:
      l(dependent.full_name_ar, dependent.full_name) ?? dependent.full_name,
    subtitle:
      dependent.current_grade_level ??
      dependent.current_section_name ??
      undefined,
    icon: <StudentBoy size={38} />,
    onPress: () =>
      router.push({
        pathname: "/(app)/student/[id]",
        params: { id: dependent.student_id },
      }),
  }));
  const teacherClasses: HomeTileData[] = (classesQuery.data ?? []).map(
    (classItem: ClassDto) => ({
      id: classItem.id,
      title: l(classItem.name_ar, classItem.name) ?? classItem.name,
      subtitle:
        classItem.current_students !== null &&
        classItem.current_students !== undefined
          ? t("home_class_students_count", {
              count: classItem.current_students,
            })
          : classItem.level,
      icon: <BookOpen size={30} color={theme.primary} />,
      onPress: () =>
        router.push({
          pathname: "/(app)/class/[id]",
          params: { id: classItem.id },
        }),
    }),
  );
  const firstDependentId = dependents[0]?.student_id;

  const serviceTiles: HomeTileData[] =
    activeRole === "parent"
      ? [
          ...(firstDependentId
            ? [
                {
                  id: "attendance",
                  title: t("student_attendance"),
                  icon: <Attendance size={38} />,
                  onPress: () =>
                    router.push({
                      pathname: "/(app)/student/[id]/attendance",
                      params: { id: firstDependentId },
                    }),
                },
                {
                  id: "grades",
                  title: t("student_grades"),
                  icon: <Grades size={38} />,
                  onPress: () =>
                    router.push({
                      pathname: "/(app)/student/[id]/grades",
                      params: { id: firstDependentId },
                    }),
                },
              ]
            : []),
          {
            id: "notifications",
            title: t("home_notifications"),
            icon: <ChatBubbles size={28} color={theme.primary} />,
            onPress: () => router.push("/(app)/notifications"),
          },
        ]
      : activeRole === "student"
        ? [
            {
              id: "schedule",
              title: t("student_schedule"),
              icon: <Schedule size={38} />,
              onPress: () => router.push("/(app)/student/me/schedule"),
            },
            {
              id: "assignments",
              title: t("student_exams"),
              icon: <AssignmentChecklist size={38} />,
              onPress: () => router.push("/(app)/student/me/exams"),
            },
            {
              id: "grades",
              title: t("student_grades"),
              icon: <Grades size={38} />,
              onPress: () => router.push("/(app)/student/me/grades"),
            },
            {
              id: "attendance",
              title: t("student_attendance"),
              icon: <Attendance size={38} />,
              onPress: () => router.push("/(app)/student/me/attendance"),
            },
          ]
        : [
            {
              id: "attendance",
              title: t("teacher_mark_attendance"),
              icon: <Attendance size={38} />,
              onPress: () => router.push("/(app)/class/attendance"),
            },
            {
              id: "grades",
              title: t("teacher_record_grades"),
              icon: <Grades size={38} />,
              onPress: () => router.push("/(app)/class/grades"),
            },
            {
              id: "notifications",
              title: t("home_notifications"),
              icon: <ChatBubbles size={28} color={theme.primary} />,
              onPress: () => router.push("/(app)/notifications"),
            },
            {
              id: "assignments",
              title: t("student_exams"),
              icon: <Assignments size={38} />,
              onPress: () => router.push("/(app)/class/homework"),
            },
          ];

  return (
    <ScreenScrollView
      edges={["bottom", "left", "right"]}
      contentContainerStyle={{
        paddingTop: insets.top + HEADER_CONTENT_OFFSET + 12,
      }}
      scrollViewProps={{
        contentContainerClassName: "gap-six px-four pb-[130px]",
      }}
    >
      <HomeHero firstName={firstName} />

      {roleOptions.length > 1 ? (
        <SegmentedControl
          options={roleOptions}
          value={activeRole}
          onChange={setSelectedRole}
        />
      ) : null}

      <HomeSection title={t("dashboard_quick_actions")}>
        <QuickActionRow items={serviceTiles} />
      </HomeSection>

      {activeRole === "parent" ? (
        <HomeSection title={t("home_dependents")}>
          {parentChildren.length > 0 ? (
            <HomeTileGrid items={parentChildren} />
          ) : (
            <EmptyState
              title={t("home_no_dependents")}
              icon={<StudentBoy size={36} color={theme.textSecondary} />}
            />
          )}
        </HomeSection>
      ) : null}

      {activeRole === "teacher" ? (
        <HomeSection title={t("home_classes")}>
          {classesQuery.isLoading ? (
            <HomeTileSkeletonGrid />
          ) : classesQuery.isError ? (
            <EmptyState
              title={t("home_classes_error")}
              actionTitle={t("home_retry")}
              onAction={() => void classesQuery.refetch()}
            />
          ) : teacherClasses.length > 0 ? (
            <HomeTileGrid items={teacherClasses} />
          ) : (
            <EmptyState title={t("home_no_classes")} icon={BookOpen} />
          )}
        </HomeSection>
      ) : null}

      {activeRole === "student" ? (
        <HomeSection title={t("student_profile")}>
          {studentQuery.isLoading ? (
            <HomeTileSkeletonGrid />
          ) : studentQuery.isError ? (
            <EmptyState
              title={t("home_student_load_error")}
              actionTitle={t("home_retry")}
              onAction={() => void studentQuery.refetch()}
            />
          ) : studentQuery.data ? (
            <HomeTileGrid
              items={[
                {
                  id: studentQuery.data.id,
                  title:
                    l(
                      studentQuery.data.name_ar ??
                        studentQuery.data.display_name,
                      studentQuery.data.display_name ??
                        studentQuery.data.full_name,
                    ) ?? studentQuery.data.full_name,
                  subtitle: studentQuery.data.student_number,
                  icon: <StudentBoy size={38} />,
                  onPress: () =>
                    router.push({
                      pathname: "/(app)/student/[id]",
                      params: { id: "me" },
                    }),
                },
              ]}
            />
          ) : (
            <EmptyState title={t("home_student_load_error")} />
          )}
        </HomeSection>
      ) : null}
    </ScreenScrollView>
  );
}

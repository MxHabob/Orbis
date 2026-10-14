import { BookOpen, ChatBubbles } from "@orbis/icons/native";
import {
  AssignmentChecklist,
  Attendance,
  Grades,
  Schedule,
  StudentBoy,
} from "@orbis/icons/native/Illustrations";
import { useRouter } from "expo-router";
import { useCallback, useMemo, useState } from "react";
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
import { useToast } from "@/components/ui/toast";
import {
  isApiNotFoundError,
  useMyStudentQuery,
} from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useEffectiveContext } from "@/providers/effective-context";

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
  const {
    user,
    effectiveContext,
    refreshSessionData,
    isLoading: isAuthLoading,
  } = useAuth();
  const {
    activeRole: contextRole,
    setActiveRole: setContextRole,
    selectedDependent,
    setSelectedDependentId,
  } = useEffectiveContext();
  const toast = useToast();

  const availableRoles = useMemo(() => {
    const detected = new Set(
      [
        effectiveContext?.effective_role ?? "",
        ...(effectiveContext?.roles ?? []),
      ]
        .map(getRoleMode)
        .filter((role): role is RoleMode => role !== null),
    );
    return (["parent", "student", "teacher"] as const).filter((role) =>
      detected.has(role),
    );
  }, [effectiveContext?.effective_role, effectiveContext?.roles]);
  const initialRole: RoleMode | null = availableRoles.includes("teacher")
    ? "teacher"
    : availableRoles.includes("student")
      ? "student"
      : (availableRoles[0] ?? null);
  const contextRoleMode = getRoleMode(contextRole);
  const activeRole =
    contextRoleMode && availableRoles.includes(contextRoleMode)
      ? contextRoleMode
      : initialRole;
  const selectRole = useCallback(
    (role: RoleMode) => {
      const roleName = [
        effectiveContext?.effective_role,
        ...(effectiveContext?.roles ?? []),
      ].find(
        (candidate) => candidate !== undefined && getRoleMode(candidate) === role,
      );
      if (roleName) setContextRole(roleName);
    },
    [effectiveContext?.effective_role, effectiveContext?.roles, setContextRole],
  );
  const studentQuery = useMyStudentQuery(
    activeRole === "student" && Boolean(effectiveContext?.student_id),
    effectiveContext?.student_id,
  );
  const [isRefreshing, setIsRefreshing] = useState(false);
  const dependents = effectiveContext?.dependents ?? [];
  const permissions = effectiveContext?.permissions ?? [];

  const refreshHome = useCallback(async () => {
    if (isRefreshing) return;
    setIsRefreshing(true);
    try {
      const refreshTasks: Promise<unknown>[] = [refreshSessionData()];
      if (activeRole === "student" && effectiveContext?.student_id) {
        refreshTasks.push(studentQuery.refetch({ throwOnError: true }));
      }
      const results = await Promise.allSettled(refreshTasks);
      const failedRefresh = results.find(
        (result): result is PromiseRejectedResult =>
          result.status === "rejected",
      );
      if (failedRefresh) throw failedRefresh.reason;
    } catch (error) {
      console.warn("[Home] Pull-to-refresh failed:", error);
      toast.error(t("home_refresh_error"));
    } finally {
      setIsRefreshing(false);
    }
  }, [
    activeRole,
    effectiveContext?.student_id,
    isRefreshing,
    refreshSessionData,
    studentQuery.refetch,
    t,
    toast,
  ]);

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
    onPress: () => {
      setSelectedDependentId(dependent.student_id);
      router.push({
        pathname: "/(app)/student/[id]",
        params: { id: dependent.student_id },
      });
    },
  }));
  const teacherClasses: HomeTileData[] = (
    effectiveContext?.teaching_classes ?? []
  )
    .filter(
      (classItem): classItem is {
        section_id: string;
        name?: string;
        name_ar?: string | null;
        level?: string | null;
      } =>
        typeof classItem === "object" &&
        classItem !== null &&
        "section_id" in classItem &&
        typeof classItem.section_id === "string",
    )
    .map((classItem) => ({
      id: classItem.section_id,
      title: l(classItem.name_ar, classItem.name) ?? classItem.name ?? "Section",
      subtitle: classItem.level ?? undefined,
      icon: <BookOpen size={30} color={theme.primary} />,
      onPress: () =>
        router.push({
          pathname: "/(app)/class/[id]",
          params: { id: classItem.section_id },
        }),
    }));
  const firstDependentId = selectedDependent?.student_id;

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
      : activeRole === "student" &&
          Boolean(effectiveContext?.student_id) &&
          !isApiNotFoundError(studentQuery.error)
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
        : activeRole === "teacher"
          ? [
              ...(teacherClasses[0] && permissions.includes("attendance:write")
                ? [
                    {
                      id: "attendance",
                      title: t("teacher_mark_attendance"),
                      icon: <Attendance size={38} />,
                      onPress: () =>
                        router.push({
                          pathname: "/(app)/class/[id]/attendance",
                          params: { id: teacherClasses[0].id },
                        }),
                    },
                    ...(permissions.includes("grade:write") ||
                    permissions.includes("grade:entry_manage")
                      ? [
                          {
                            id: "grades",
                            title: t("teacher_record_grades"),
                            icon: <Grades size={38} />,
                            onPress: () =>
                              router.push({
                                pathname: "/(app)/class/[id]/grades",
                                params: { id: teacherClasses[0].id },
                              }),
                          },
                        ]
                      : []),
                  ]
                : []),
              {
                id: "notifications",
                title: t("home_notifications"),
                icon: <ChatBubbles size={28} color={theme.primary} />,
                onPress: () => router.push("/(app)/notifications"),
              },
            ]
        : [];

  return (
    <ScreenScrollView
      edges={["bottom", "left", "right"]}
      refreshing={isRefreshing}
      onRefresh={refreshHome}
      refreshControlProps={{
        tintColor: theme.primary,
        colors: [theme.primary],
        progressBackgroundColor: theme.backgroundElement,
      }}
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
          value={activeRole ?? roleOptions[0].value}
          onChange={selectRole}
        />
      ) : null}

      {activeRole && serviceTiles.length > 0 ? (
        <HomeSection title={t("dashboard_quick_actions")}>
          <QuickActionRow items={serviceTiles} />
        </HomeSection>
      ) : null}

      {!activeRole && !isAuthLoading ? (
        <EmptyState
          title={t("home_role_unavailable")}
          icon={<StudentBoy size={36} color={theme.textSecondary} />}
        />
      ) : null}

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
          {isAuthLoading ? (
            <HomeTileSkeletonGrid />
          ) : teacherClasses.length > 0 ? (
            <HomeTileGrid items={teacherClasses} />
          ) : (
            <EmptyState title={t("home_no_classes")} icon={BookOpen} />
          )}
        </HomeSection>
      ) : null}

      {activeRole === "student" ? (
        <HomeSection title={t("student_profile")}>
          {isAuthLoading ? (
            <HomeTileSkeletonGrid />
          ) : !effectiveContext?.student_id ? (
            <EmptyState
              title={t("home_no_student_profile")}
              icon={<StudentBoy size={36} color={theme.textSecondary} />}
            />
          ) : studentQuery.isLoading ? (
            <HomeTileSkeletonGrid />
          ) : studentQuery.isError && isApiNotFoundError(studentQuery.error) ? (
            <EmptyState
              title={t("home_no_student_profile")}
              icon={<StudentBoy size={36} color={theme.textSecondary} />}
            />
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

import {
  ALL_AVATAR_HATS,
  AvatarController,
  type AvatarHat,
  Avatar as CompanionAvatar,
} from "@orbis/avatar-engine/native";
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
import { type ReactNode, useEffect, useMemo, useState } from "react";
import { ScrollView, Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { HEADER_CONTENT_OFFSET } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { SegmentedControl } from "@/components/ui/segmented-control";
import { Skeleton } from "@/components/ui/skeleton";
import { useClassesQuery, useMyStudentQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import type { ClassDto } from "@/lib/api/types";
import { useI18n } from "@/lib/i18n";

type RoleMode = "parent" | "student" | "teacher";

type HomeTileData = {
  id: string;
  title: string;
  subtitle?: string;
  icon: ReactNode;
  onPress: () => void;
};

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

function pickNextHat(
  currentHat?: AvatarHat,
): Exclude<AvatarHat, "random" | "none"> {
  const candidates = ALL_AVATAR_HATS.filter(
    (hat): hat is Exclude<AvatarHat, "random" | "none"> =>
      hat !== currentHat && hat !== "none",
  );
  return (
    candidates[Math.floor(Math.random() * candidates.length)] ?? "graduation"
  );
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
  const [currentHour, setCurrentHour] = useState(() => new Date().getHours());
  const [avatarHat, setAvatarHat] = useState<AvatarHat>(() => pickNextHat());
  const [avatarController] = useState(() => new AvatarController());
  const classesQuery = useClassesQuery(activeRole === "teacher");
  const studentQuery = useMyStudentQuery(activeRole === "student");
  const dependents = effectiveContext?.dependents ?? [];

  const roleDescription =
    activeRole === "parent"
      ? t("home_parent_description")
      : activeRole === "student"
        ? t("home_student_description")
        : t("home_teacher_description");
  const displayName = isArabic
    ? user?.full_name_ar || user?.full_name
    : user?.full_name || user?.full_name_ar;
  const firstName = displayName?.trim().split(/\s+/)[0];
  const greetingKey =
    currentHour >= 4 && currentHour < 12
      ? "home_greeting_morning"
      : currentHour >= 12 && currentHour < 17
        ? "home_greeting_afternoon"
        : "home_greeting_evening";
  const greeting = `${t(greetingKey)}${
    firstName ? `${isArabic ? "، " : ", "}${firstName}` : ""
  }`;
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

  useEffect(() => {
    const interval = setInterval(
      () => setCurrentHour(new Date().getHours()),
      60_000,
    );
    return () => clearInterval(interval);
  }, []);

  useEffect(
    () => () => {
      avatarController.destroy();
    },
    [avatarController],
  );

  const handleAvatarPress = () => {
    setAvatarHat((current) => pickNextHat(current));
    const mood = Math.random();
    if (mood < 0.34) avatarController.happy();
    else if (mood < 0.67) avatarController.think();
    else avatarController.blink();
  };

  return (
    <ScreenScrollView
      edges={["bottom", "left", "right"]}
      contentContainerStyle={{
        paddingTop: insets.top + HEADER_CONTENT_OFFSET,
      }}
      scrollViewProps={{
        contentContainerClassName: "gap-six px-four pb-[130px]",
      }}
    >
      <View className="items-center gap-three px-two pb-one pt-one">
        <CompanionAvatar
          size={112}
          controller={avatarController}
          hat={avatarHat}
          label={t("home_avatar_change_hat")}
          accessibilityHint={t("home_avatar_hint")}
          onPress={handleAvatarPress}
        />
        <Text className="text-xs font-medium text-muted-foreground">
          {t("home_avatar_hint")}
        </Text>
        <View className="items-center gap-two">
          <Text
            numberOfLines={2}
            className="text-center text-[26px] font-bold tracking-tight text-foreground"
          >
            {greeting}
          </Text>
          <Text className="max-w-[320px] text-center text-sm leading-5 text-muted-foreground">
            {roleDescription}
          </Text>
        </View>
      </View>

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

function HomeSection({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  return (
    <View className="gap-three">
      <Text className="text-base font-semibold tracking-tight text-foreground">
        {title}
      </Text>
      {children}
    </View>
  );
}

function QuickActionRow({ items }: { items: HomeTileData[] }) {
  return (
    <ScrollView
      horizontal
      nestedScrollEnabled
      showsHorizontalScrollIndicator={false}
      contentContainerClassName="gap-three px-one"
    >
      {items.map((item) => (
        <Card
          key={item.id}
          variant="interactive"
          padding="three"
          onPress={item.onPress}
          accessibilityRole="button"
          accessibilityLabel={item.title}
          style={{ width: 168, minHeight: 76 }}
        >
          <View className="flex-1 flex-row items-center gap-three">
            <View className="h-10 w-10 items-center justify-center rounded-md bg-primary/10">
              {item.icon}
            </View>
            <Text
              numberOfLines={2}
              className="flex-1 text-sm font-semibold leading-5 text-foreground"
            >
              {item.title}
            </Text>
          </View>
        </Card>
      ))}
    </ScrollView>
  );
}

function HomeTileGrid({ items }: { items: HomeTileData[] }) {
  return (
    <View className="flex-row flex-wrap justify-between gap-y-three">
      {items.map((item) => (
        <Card
          key={item.id}
          variant="interactive"
          padding="four"
          onPress={item.onPress}
          accessibilityRole="button"
          accessibilityLabel={item.title}
          style={{ width: "48%", minHeight: 108 }}
        >
          <View className="flex-1 justify-between gap-three">
            <View className="h-11 w-11 items-center justify-center rounded-md bg-primary/10">
              {item.icon}
            </View>
            <Text
              numberOfLines={1}
              className="text-sm font-semibold leading-5 text-foreground"
            >
              {item.title}
            </Text>
            {item.subtitle ? (
              <Text
                numberOfLines={1}
                className="text-xs leading-4 text-muted-foreground"
              >
                {item.subtitle}
              </Text>
            ) : null}
          </View>
        </Card>
      ))}
    </View>
  );
}

function HomeTileSkeletonGrid() {
  return (
    <View className="flex-row flex-wrap justify-between gap-y-three">
      {["first", "second"].map((key) => (
        <Skeleton key={key} width="48%" height={108} borderRadius={8} />
      ))}
    </View>
  );
}

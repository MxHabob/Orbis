import { type ReactNode, useState } from "react";
import { Pressable, ScrollView, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { HEADER_CONTENT_OFFSET } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { Avatar as CompanionAvatar } from "@orbis/avatar-engine/native";
import { ChatBubbles } from "@orbis/icons/native";
import {
  AssignmentChecklist,
  Assignments,
  Attendance,
  English,
  Grade7Math,
  Grade8Science,
  Grade10Math,
  Grades,
  Schedule,
  StudentBoy,
  StudentGirl,
} from "@orbis/icons/native/Illustrations";

type RoleMode = "parent" | "student" | "teacher";

type HomeTileData = {
  id: string;
  title: string;
  icon: ReactNode;
  onPress: () => void;
};

export default function HomeTabScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
  const { user, effectiveContext } = useAuth();

  const roles = new Set(
    [
      ...(user?.roles ?? []),
      effectiveContext?.effective_role ?? "",
      ...(effectiveContext?.roles ?? []),
    ].map((role) => role.toLowerCase().replace(/[^a-z]/g, "")),
  );

  const initialRole: RoleMode = [...roles].some((role) =>
    role.includes("teacher"),
  )
    ? "teacher"
    : [...roles].some((role) => role.includes("student"))
      ? "student"
      : "parent";

  const [activeRole, setActiveRole] = useState<RoleMode>(initialRole);

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

  const parentChildren: HomeTileData[] = [
    {
      id: "omar",
      title: t("home_child_omar"),
      icon: <StudentBoy size={40} />,
      onPress: () => router.push("/(app)/student/omar"),
    },
    {
      id: "lina",
      title: t("home_child_lina"),
      icon: <StudentGirl size={40} />,
      onPress: () => router.push("/(app)/student/lina"),
    },
  ];

  const teacherClasses: HomeTileData[] = [
    {
      id: "grade-seven",
      title: t("home_class_seven_math"),
      icon: <Grade7Math size={40} />,
      onPress: () => router.push("/(app)/class/7"),
    },
    {
      id: "grade-eight",
      title: t("home_class_eight_science"),
      icon: <Grade8Science size={40} />,
      onPress: () => router.push("/(app)/class/8"),
    },
    {
      id: "grade-nine",
      title: t("home_class_nine_english"),
      icon: <English size={40} />,
      onPress: () => router.push("/(app)/class/9"),
    },
    {
      id: "grade-ten",
      title: t("home_class_ten_math"),
      icon: <Grade10Math size={40} />,
      onPress: () => router.push("/(app)/class/10"),
    },
  ];

  const serviceTiles: HomeTileData[] =
    activeRole === "parent"
      ? [
          {
            id: "attendance",
            title: t("student_attendance"),
            icon: <Attendance size={38} />,
            onPress: () => router.push("/(app)/student/omar/attendance"),
          },
          {
            id: "grades",
            title: t("student_grades"),
            icon: <Grades size={38} />,
            onPress: () => router.push("/(app)/student/omar/grades"),
          },
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
        paddingTop: insets.top + HEADER_CONTENT_OFFSET,
      }}
      scrollViewProps={{
        contentContainerClassName: "gap-six px-four pb-[130px]",
      }}
    >
      <View className="items-center gap-two px-two pt-one">
        <CompanionAvatar size={58} hat="random" label={t("nav_app_name")} />
        <View className="items-center gap-one">
          <Text className="text-sm font-medium text-muted-foreground">
            {t("home_welcome")}
          </Text>
          {firstName ? (
            <Text
              numberOfLines={1}
              className="text-2xl font-bold tracking-tight text-foreground"
            >
              {firstName}
            </Text>
          ) : null}
          <Text className="max-w-[320px] text-center text-sm leading-5 text-muted-foreground">
            {roleDescription}
          </Text>
        </View>
      </View>

      <View
        accessibilityRole="tablist"
        className="flex-row rounded-lg border border-border bg-background-element p-half"
      >
        {(
          [
            ["parent", "auth_role_parent"],
            ["student", "auth_role_student"],
            ["teacher", "auth_role_teacher"],
          ] as const
        ).map(([role, labelKey]) => {
          const selected = activeRole === role;
          return (
            <Pressable
              key={role}
              accessibilityRole="tab"
              accessibilityState={{ selected }}
              onPress={() => setActiveRole(role)}
              className={`min-h-11 flex-1 items-center justify-center rounded-md px-two ${
                selected ? "border border-card-border bg-card-elevated" : ""
              }`}
            >
              <Text
                numberOfLines={1}
                className={`text-xs font-semibold ${
                  selected ? "text-foreground" : "text-muted-foreground"
                }`}
              >
                {t(labelKey)}
              </Text>
            </Pressable>
          );
        })}
      </View>

      <HomeSection title={t("dashboard_quick_actions")}>
        <QuickActionRow items={serviceTiles} />
      </HomeSection>

      {activeRole === "parent" ? (
        <HomeSection title={t("home_dependents")}>
          <HomeTileGrid items={parentChildren} />
        </HomeSection>
      ) : null}

      {activeRole === "teacher" ? (
        <HomeSection title={t("home_classes")}>
          <HomeTileGrid items={teacherClasses} />
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
          style={{ width: 176, minHeight: 72 }}
        >
          <View className="flex-1 flex-row items-center gap-three">
            <View className="h-10 w-10 items-center justify-center rounded-md bg-accent-light">
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
          style={{ width: "48%", minHeight: 112 }}
        >
          <View className="flex-1 justify-between gap-three">
            <View className="h-11 w-11 items-center justify-center rounded-md bg-accent-light">
              {item.icon}
            </View>
            <Text
              numberOfLines={2}
              className="text-sm font-semibold leading-5 text-foreground"
            >
              {item.title}
            </Text>
          </View>
        </Card>
      ))}
    </View>
  );
}

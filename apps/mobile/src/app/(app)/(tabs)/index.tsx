/**
 * @module @orbis/mobile/app/(app)/(tabs)/index
 *
 * Primary Workspace & Apps Hub for Orbis Mobile.
 * Adapts dynamically based on active user role:
 * - Parents / Guardians: Dependent child selector, daily academic schedule, attendance & fee shortcuts.
 * - Teachers: Class roster summaries, quick roll-call attendance, and grade entry shortcuts.
 * - Students: My profile, schedule, grades, and exams.
 */

import React, { useState } from "react";
import { View, Text, Pressable } from "react-native";
import { useRouter } from "expo-router";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { Avatar } from "@/components/ui/avatar";
import { useI18n } from "@/lib/i18n";
import { useAuth } from "@/lib/api";
import { useTheme } from "@/hooks/use-theme";
import { useClassesQuery } from "@/hooks/use-api-queries";
import {
  AcademicCap,
  Calendar,
  ChartBar,
  Document,
  User,
  ChevronLeft,
  ChevronRight,
  Users,
} from "@orbis/icons/native";

export default function HomeTabScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic, formatDate } = useI18n();
  const { user, effectiveContext } = useAuth();

  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

  // Extract normalized roles
  const roles = new Set(
    [
      ...(user?.roles ?? []),
      effectiveContext?.effective_role ?? "",
      ...(effectiveContext?.roles ?? []),
    ].map((role) => role.toLowerCase().replace(/[^a-z]/g, "")),
  );

  const isGuardian = [...roles].some(
    (role) => role.includes("parent") || role.includes("guardian"),
  );
  const isTeacher = [...roles].some((role) => role.includes("teacher"));
  const isStudent = [...roles].some((role) => role.includes("student"));

  const dependents = effectiveContext?.dependents ?? [];
  const [selectedDependentIndex, setSelectedDependentIndex] = useState(0);
  const activeDependent = dependents[selectedDependentIndex] ?? dependents[0];

  const classesQuery = useClassesQuery(isTeacher);

  const fullName = isArabic
    ? user?.full_name_ar || user?.full_name || "مستخدم أوربيس"
    : user?.full_name || "Orbis User";

  const orgName = isArabic
    ? effectiveContext?.organization_name_ar || effectiveContext?.organization_name
    : effectiveContext?.organization_name;

  return (
    <ScreenScrollView
      edges={["bottom", "left", "right"]}
      scrollViewProps={{
        contentContainerClassName: "gap-six px-four pb-[130px] pt-one",
      }}
      refreshing={classesQuery.isRefetching}
      onRefresh={isTeacher ? () => void classesQuery.refetch() : undefined}
    >
      {/* 1. Hero Greeting & Organization Badge */}
      <View className="gap-two pt-one">
        <View className="flex-row items-center justify-between">
          <View className="flex-row items-center gap-two">
            {orgName ? (
              <Badge variant="secondary" className="px-three py-half">
                <Text className="text-xs font-semibold text-primary">{orgName}</Text>
              </Badge>
            ) : null}
            <Text className="text-xs font-medium text-muted-foreground">
              {formatDate(new Date())}
            </Text>
          </View>
        </View>

        <View className="gap-half">
          <Text className="text-sm font-medium text-muted-foreground">
            {t("home_welcome")}
          </Text>
          <Text className="text-2xl font-extrabold tracking-tight text-foreground">
            {fullName}
          </Text>
        </View>
      </View>

      {/* 2. Guardian Dependent Selector & Quick Jump */}
      {isGuardian && dependents.length > 0 ? (
        <View className="gap-three">
          <View className="flex-row items-center justify-between">
            <Text className="text-base font-bold text-foreground">
              {t("home_dependents")}
            </Text>
            {dependents.length > 1 ? (
              <Text className="text-xs text-muted-foreground">
                {selectedDependentIndex + 1} / {dependents.length}
              </Text>
            ) : null}
          </View>

          {/* Child Switcher Chips if more than 1 */}
          {dependents.length > 1 ? (
            <View className="flex-row gap-two">
              {dependents.map((dep, idx) => {
                const isSelected = idx === selectedDependentIndex;
                const depName = isArabic
                  ? dep.full_name_ar || dep.full_name
                  : dep.full_name;
                return (
                  <Pressable
                    key={dep.student_id}
                    onPress={() => setSelectedDependentIndex(idx)}
                    className={`rounded-full px-four py-two transition-all ${
                      isSelected
                        ? "bg-primary"
                        : "border border-border bg-card"
                    }`}
                  >
                    <Text
                      className={`text-xs font-bold ${
                        isSelected ? "text-primary-foreground" : "text-foreground"
                      }`}
                    >
                      {depName}
                    </Text>
                  </Pressable>
                );
              })}
            </View>
          ) : null}

          {/* Active Child Overview Card */}
          {activeDependent ? (
            <Card variant="default" padding="four" className="bg-card shadow-sm border border-border/80">
              <View className="flex-row items-center justify-between pb-three">
                <View className="flex-row items-center gap-three">
                  <Avatar
                    fallback={(activeDependent.full_name || "S").charAt(0)}
                    size="md"
                    className="bg-primary/10"
                  />
                  <View className="gap-half">
                    <Text className="text-base font-bold text-foreground">
                      {isArabic
                        ? activeDependent.full_name_ar || activeDependent.full_name
                        : activeDependent.full_name}
                    </Text>
                    <Text className="text-xs font-medium text-muted-foreground">
                      {[
                        activeDependent.current_grade_level,
                        activeDependent.current_section_name,
                      ]
                        .filter(Boolean)
                        .join(" · ")}
                    </Text>
                  </View>
                </View>

                <Pressable
                  accessibilityRole="button"
                  onPress={() =>
                    router.push({
                      pathname: "/(app)/student/[id]",
                      params: { id: activeDependent.student_id },
                    })
                  }
                  className="rounded-full bg-secondary p-two"
                >
                  <DirectionChevron size={18} color={theme.foreground} />
                </Pressable>
              </View>

              {/* Sub-module Shortcuts for Active Child */}
              <View className="mt-two flex-row justify-between border-t border-border/60 pt-three">
                <QuickShortcutButton
                  icon={<ChartBar size={18} color={theme.primary} />}
                  label={t("domain_grades")}
                  onPress={() =>
                    router.push({
                      pathname: "/(app)/student/[id]/grades",
                      params: { id: activeDependent.student_id },
                    })
                  }
                />
                <QuickShortcutButton
                  icon={<Document size={18} color={theme.primary} />}
                  label={t("domain_attendance")}
                  onPress={() =>
                    router.push({
                      pathname: "/(app)/student/[id]/attendance",
                      params: { id: activeDependent.student_id },
                    })
                  }
                />
                <QuickShortcutButton
                  icon={<Calendar size={18} color={theme.primary} />}
                  label={t("domain_schedule")}
                  onPress={() =>
                    router.push({
                      pathname: "/(app)/student/[id]/schedule",
                      params: { id: activeDependent.student_id },
                    })
                  }
                />
                <QuickShortcutButton
                  icon={<AcademicCap size={18} color={theme.primary} />}
                  label={t("student_exams")}
                  onPress={() =>
                    router.push({
                      pathname: "/(app)/student/[id]/exams",
                      params: { id: activeDependent.student_id },
                    })
                  }
                />
              </View>
            </Card>
          ) : null}
        </View>
      ) : null}

      {/* 3. Teacher Classes Hub */}
      {isTeacher ? (
        <View className="gap-three">
          <View className="flex-row items-center justify-between">
            <Text className="text-base font-bold text-foreground">
              {t("home_classes")}
            </Text>
            {classesQuery.data?.length ? (
              <Badge variant="outline" className="px-two py-half">
                <Text className="text-xs font-semibold text-muted-foreground">
                  {classesQuery.data.length} {t("nav_classes") || "Classes"}
                </Text>
              </Badge>
            ) : null}
          </View>

          {classesQuery.isPending ? (
            <View className="gap-three">
              <Skeleton className="h-28 w-full rounded-2xl" />
              <Skeleton className="h-28 w-full rounded-2xl" />
            </View>
          ) : classesQuery.isError ? (
            <Card variant="outline" padding="four" className="border-destructive/30">
              <Text className="text-sm text-destructive">{t("home_classes_error")}</Text>
              <Pressable
                onPress={() => void classesQuery.refetch()}
                className="mt-two self-start rounded-lg bg-primary px-three py-one"
              >
                <Text className="text-xs font-bold text-primary-foreground">{t("home_retry")}</Text>
              </Pressable>
            </Card>
          ) : classesQuery.data?.length ? (
            classesQuery.data.map((c) => {
              const className = isArabic ? c.name_ar || c.name : c.name;
              return (
                <Card
                  key={c.id}
                  variant="default"
                  padding="four"
                  className="bg-card border border-border/80 shadow-sm"
                >
                  <View className="flex-row items-center justify-between pb-three">
                    <View className="gap-half">
                      <Text className="text-base font-bold text-foreground">
                        {className}
                      </Text>
                      <Text className="text-xs text-muted-foreground">
                        {t("home_class_level")} {c.level}
                      </Text>
                    </View>
                    <Pressable
                      accessibilityRole="button"
                      onPress={() =>
                        router.push({
                          pathname: "/(app)/class/[id]",
                          params: { id: c.id },
                        })
                      }
                      className="rounded-full bg-secondary p-two"
                    >
                      <DirectionChevron size={18} color={theme.foreground} />
                    </Pressable>
                  </View>

                  {/* Direct Teacher Shortcuts */}
                  <View className="flex-row items-center gap-two border-t border-border/60 pt-three">
                    <Pressable
                      onPress={() =>
                        router.push({
                          pathname: "/(app)/class/[id]/attendance",
                          params: { id: c.id },
                        })
                      }
                      className="flex-1 flex-row items-center justify-center gap-one rounded-lg bg-primary/10 py-two"
                    >
                      <Document size={16} color={theme.primary} />
                      <Text className="text-xs font-bold text-primary">
                        {t("teacher_mark_attendance")}
                      </Text>
                    </Pressable>

                    <Pressable
                      onPress={() =>
                        router.push({
                          pathname: "/(app)/class/[id]/grades",
                          params: { id: c.id },
                        })
                      }
                      className="flex-1 flex-row items-center justify-center gap-one rounded-lg bg-secondary py-two"
                    >
                      <ChartBar size={16} color={theme.foreground} />
                      <Text className="text-xs font-bold text-foreground">
                        {t("teacher_record_grades")}
                      </Text>
                    </Pressable>
                  </View>
                </Card>
              );
            })
          ) : (
            <Card variant="outline" padding="four">
              <Text className="text-center text-sm text-muted-foreground">
                {t("home_no_classes")}
              </Text>
            </Card>
          )}
        </View>
      ) : null}

      {/* 4. Student Self-Service Hub */}
      {isStudent ? (
        <View className="gap-three">
          <Text className="text-base font-bold text-foreground">
            {t("student_profile")}
          </Text>
          <Card
            variant="default"
            padding="four"
            className="bg-card border border-border/80 shadow-sm"
          >
            <View className="flex-row items-center justify-between pb-three">
              <View className="gap-half">
                <Text className="text-base font-bold text-foreground">
                  {fullName}
                </Text>
                <Text className="text-xs text-muted-foreground">
                  {t("student_profile")}
                </Text>
              </View>
              <Pressable
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]",
                    params: { id: "me" },
                  })
                }
                className="rounded-full bg-secondary p-two"
              >
                <DirectionChevron size={18} color={theme.foreground} />
              </Pressable>
            </View>

            <View className="mt-two flex-row justify-between border-t border-border/60 pt-three">
              <QuickShortcutButton
                icon={<ChartBar size={18} color={theme.primary} />}
                label={t("domain_grades")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/grades",
                    params: { id: "me" },
                  })
                }
              />
              <QuickShortcutButton
                icon={<Document size={18} color={theme.primary} />}
                label={t("domain_attendance")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/attendance",
                    params: { id: "me" },
                  })
                }
              />
              <QuickShortcutButton
                icon={<Calendar size={18} color={theme.primary} />}
                label={t("domain_schedule")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/schedule",
                    params: { id: "me" },
                  })
                }
              />
              <QuickShortcutButton
                icon={<AcademicCap size={18} color={theme.primary} />}
                label={t("student_exams")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]/exams",
                    params: { id: "me" },
                  })
                }
              />
            </View>
          </Card>
        </View>
      ) : null}

      {/* 5. Quick Tools & Institutional Shortcuts */}
      <View className="gap-three">
        <Text className="text-base font-bold text-foreground">
          {t("home_services")}
        </Text>

        <View className="gap-two">
          <ServiceRowItem
            icon={<Users size={20} color={theme.primary} />}
            title={t("tab_digital_id") || "Digital School ID"}
            description={t("tab_digital_id_hint") || "Campus entry pass & QR code"}
            onPress={() => router.push("/(app)/(modal)/Id")}
            chevron={DirectionChevron}
          />
          <ServiceRowItem
            icon={<User size={20} color={theme.primary} />}
            title={t("nav_profile")}
            description={t("home_profile_description")}
            onPress={() => router.push("/(app)/profile")}
            chevron={DirectionChevron}
          />
        </View>
      </View>
    </ScreenScrollView>
  );
}

function QuickShortcutButton({
  icon,
  label,
  onPress,
}: {
  icon: React.ReactNode;
  label: string;
  onPress: () => void;
}) {
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      className="items-center gap-one px-two active:opacity-70"
    >
      <View className="h-10 w-10 items-center justify-center rounded-xl bg-secondary">
        {icon}
      </View>
      <Text className="text-xs font-semibold text-foreground">{label}</Text>
    </Pressable>
  );
}

function ServiceRowItem({
  icon,
  title,
  description,
  onPress,
  chevron: Chevron,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  onPress: () => void;
  chevron: any;
}) {
  const theme = useTheme();
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      className="flex-row items-center justify-between rounded-2xl border border-border/80 bg-card p-three shadow-xs active:bg-secondary/40"
    >
      <View className="flex-row items-center gap-three">
        <View className="h-11 w-11 items-center justify-center rounded-xl bg-primary/10">
          {icon}
        </View>
        <View className="gap-half">
          <Text className="text-sm font-bold text-foreground">{title}</Text>
          <Text className="text-xs text-muted-foreground">{description}</Text>
        </View>
      </View>
      <Chevron size={18} color={theme.mutedForeground} />
    </Pressable>
  );
}
